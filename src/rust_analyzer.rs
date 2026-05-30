use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use tracing::{debug, warn};

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub insert_text: String,
}

#[derive(Debug)]
pub enum LspEvent {
    InitializeComplete,
    CompletionResponse {
        request_id: u64,
        items: Vec<CompletionItem>,
    },
    HoverResponse {
        request_id: u64,
        contents: Option<String>,
    },
}

enum OutboundMessage {
    Json(Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PendingRequestKind {
    Completion,
    Hover,
}

#[derive(Debug, Clone)]
struct LaunchSpec {
    program: String,
    args: Vec<String>,
}

pub struct RustAnalyzerClient {
    _child: Child,
    tx: Sender<OutboundMessage>,
    rx: Receiver<LspEvent>,
    pending_requests: Arc<Mutex<std::collections::HashMap<u64, PendingRequestKind>>>,
    analyzer_state: Arc<(Mutex<AnalyzerState>, Condvar)>,
    launch_command: String,
    uri: String,
    root_uri: String,
    version: i32,
    next_request_id: u64,
}

#[derive(Debug, Default)]
struct AnalyzerState {
    pending_work: usize,
    ready: bool,
    progress_seen: bool,
    server_status_seen: bool,
}

impl RustAnalyzerClient {
    pub fn start(file_path: &Path) -> io::Result<Self> {
        Self::start_with_trace(file_path, false)
    }

    pub fn start_with_trace(file_path: &Path, trace_io: bool) -> io::Result<Self> {
        let launch = choose_launch_spec()?;
        let launch_command = format_launch_spec(&launch);

        let mut command = Command::new(&launch.program);
        command.args(&launch.args);

        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|err| io::Error::new(err.kind(), format!("failed to launch {launch_command}: {err}")))?;

        let Some(stdin) = child.stdin.take() else {
            return Err(io::Error::other("failed to capture rust-analyzer stdin"));
        };
        let Some(stdout) = child.stdout.take() else {
            return Err(io::Error::other("failed to capture rust-analyzer stdout"));
        };

        let (tx_out, rx_out) = mpsc::channel::<OutboundMessage>();
        let (tx_evt, rx_evt) = mpsc::channel::<LspEvent>();
        let pending_requests = Arc::new(Mutex::new(std::collections::HashMap::new()));
        let analyzer_state = Arc::new((Mutex::new(AnalyzerState::default()), Condvar::new()));

        thread::spawn(move || writer_thread(stdin, rx_out, trace_io));
        thread::spawn({
            let pending_requests = Arc::clone(&pending_requests);
            let analyzer_state = Arc::clone(&analyzer_state);
            let tx_out = tx_out.clone();
            move || reader_thread(stdout, tx_evt, tx_out, pending_requests, analyzer_state, trace_io)
        });

        let uri = path_to_file_uri(file_path)?;
        let workspace_root = absolute_path(&detect_workspace_root(file_path)?)?;
        let root_uri = path_to_file_uri(&workspace_root)?;
        let workspace_name = workspace_root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("workspace")
            .to_string();
        let root_path = workspace_root.to_string_lossy().to_string();

        let client = Self {
            _child: child,
            tx: tx_out,
            rx: rx_evt,
            pending_requests,
            analyzer_state,
            launch_command: launch_command.clone(),
            uri,
            root_uri: root_uri.clone(),
            version: 1,
            next_request_id: 2,
        };

        client.send_json(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": std::process::id(),
                "rootPath": root_path,
                "rootUri": root_uri.clone(),
                "workspaceFolders": [{
                    "uri": root_uri.clone(),
                    "name": workspace_name
                }],
                "capabilities": {
                    "window": {
                        "workDoneProgress": true
                    },
                    "workspace": {
                        "workspaceFolders": true
                    },
                    "textDocument": {
                        "completion": {
                            "completionItem": {
                                "snippetSupport": false
                            }
                        }
                    }
                },
                "clientInfo": {
                    "name": "spf-edit",
                    "version": "0.1.0"
                }
            }
        }))?;

        match client.rx.recv_timeout(Duration::from_secs(5)) {
            Ok(LspEvent::InitializeComplete) => {}
            Ok(other) => {
                warn!("unexpected LSP event during initialize: {:?}", other);
            }
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    format!(
                        "timed out waiting for rust-analyzer initialize response from {launch_command}"
                    ),
                ));
            }
        }

        client.send_json(json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        }))?;

        Ok(client)
    }

    pub fn did_open(&self, text: &str) -> io::Result<()> {
        self.mark_analyzer_not_ready();
        self.send_json(json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": self.uri,
                    "languageId": "rust",
                    "version": self.version,
                    "text": text
                }
            }
        }))
    }

    pub fn did_change(&mut self, text: &str) -> io::Result<i32> {
        self.mark_analyzer_not_ready();
        self.version += 1;
        self.send_json(json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": {
                    "uri": self.uri,
                    "version": self.version
                },
                "contentChanges": [{
                    "text": text
                }]
            }
        }))?;
        Ok(self.version)
    }

    pub fn request_completion(
        &mut self,
        line: u32,
        character: u32,
        trigger_character: Option<char>,
    ) -> io::Result<u64> {
        let request_id = self.next_request_id;
        self.next_request_id += 1;
        self.record_request_kind(request_id, PendingRequestKind::Completion);
        let trigger_kind = if trigger_character.is_some() { 2 } else { 1 };
        self.send_json(json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "textDocument/completion",
            "params": {
                "textDocument": { "uri": self.uri },
                "position": {
                    "line": line,
                    "character": character
                },
                "context": {
                    "triggerKind": trigger_kind,
                    "triggerCharacter": trigger_character.map(|ch| ch.to_string())
                }
            }
        }))?;
        Ok(request_id)
    }

    pub fn request_hover(&mut self, line: u32, character: u32) -> io::Result<u64> {
        let request_id = self.next_request_id;
        self.next_request_id += 1;
        self.record_request_kind(request_id, PendingRequestKind::Hover);
        self.send_json(json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "textDocument/hover",
            "params": {
                "textDocument": { "uri": self.uri },
                "position": {
                    "line": line,
                    "character": character
                }
            }
        }))?;
        Ok(request_id)
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn document_uri(&self) -> &str {
        &self.uri
    }

    pub fn launch_command(&self) -> &str {
        &self.launch_command
    }

    pub fn workspace_root_uri(&self) -> &str {
        &self.root_uri
    }

    pub fn wait_for_project_ready(&self, timeout: Duration) -> io::Result<bool> {
        let deadline = Instant::now() + timeout;
        let (lock, cvar) = &*self.analyzer_state;
        let mut state = lock
            .lock()
            .map_err(|_| io::Error::other("analyzer state lock poisoned"))?;

        if state.ready {
            return Ok(true);
        }

        loop {
            let now = Instant::now();
            if now >= deadline {
                return Ok(state.ready);
            }

            let remaining = deadline.saturating_duration_since(now);
            let (next_state, wait_result) = cvar
                .wait_timeout(state, remaining)
                .map_err(|_| io::Error::other("analyzer state wait poisoned"))?;
            state = next_state;

            if state.ready {
                return Ok(true);
            }

            if wait_result.timed_out() {
                return Ok(state.ready);
            }
        }
    }

    pub fn try_recv(&self) -> Result<LspEvent, TryRecvError> {
        self.rx.try_recv()
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Result<LspEvent, mpsc::RecvTimeoutError> {
        self.rx.recv_timeout(timeout)
    }

    fn record_request_kind(&self, request_id: u64, kind: PendingRequestKind) {
        if let Ok(mut pending) = self.pending_requests.lock() {
            pending.insert(request_id, kind);
        }
    }

    fn mark_analyzer_not_ready(&self) {
        let (lock, cvar) = &*self.analyzer_state;
        if let Ok(mut state) = lock.lock() {
            state.ready = false;
            state.pending_work = 0;
            state.progress_seen = false;
            state.server_status_seen = false;
            cvar.notify_all();
        }
    }

    fn send_json(&self, value: Value) -> io::Result<()> {
        self.tx
            .send(OutboundMessage::Json(value))
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "LSP channel disconnected"))
    }
}

fn writer_thread(mut stdin: ChildStdin, rx: Receiver<OutboundMessage>, trace_io: bool) {
    while let Ok(msg) = rx.recv() {
        let OutboundMessage::Json(value) = msg;
        if trace_io {
            eprintln!("LSP -> {}", value);
        }
        let Ok(payload) = serde_json::to_vec(&value) else {
            continue;
        };
        let header = format!("Content-Length: {}\r\n\r\n", payload.len());
        if stdin
            .write_all(header.as_bytes())
            .and_then(|_| stdin.write_all(&payload))
            .and_then(|_| stdin.flush())
            .is_err()
        {
            break;
        }
    }
}

fn reader_thread(
    stdout: ChildStdout,
    tx: Sender<LspEvent>,
    tx_out: Sender<OutboundMessage>,
    pending_requests: Arc<Mutex<std::collections::HashMap<u64, PendingRequestKind>>>,
    analyzer_state: Arc<(Mutex<AnalyzerState>, Condvar)>,
    trace_io: bool,
) {
    let mut reader = BufReader::new(stdout);
    loop {
        let Ok(Some(content_length)) = read_headers(&mut reader) else {
            break;
        };

        let mut payload = vec![0u8; content_length];
        if reader.read_exact(&mut payload).is_err() {
            break;
        }

        let Ok(msg): Result<Value, _> = serde_json::from_slice(&payload) else {
            continue;
        };
        if trace_io {
            eprintln!("LSP <- {}", msg);
        }
        handle_inbound_message(msg, &tx, &tx_out, &pending_requests, &analyzer_state);
    }
}

fn read_headers<R: BufRead>(reader: &mut R) -> io::Result<Option<usize>> {
    let mut content_length: Option<usize> = None;
    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Ok(None);
        }
        if line == "\r\n" || line == "\n" {
            return Ok(content_length);
        }

        if let Some(value) = line.strip_prefix("Content-Length:") {
            if let Ok(len) = value.trim().parse::<usize>() {
                content_length = Some(len);
            }
        }
    }
}

fn handle_inbound_message(
    message: Value,
    tx: &Sender<LspEvent>,
    tx_out: &Sender<OutboundMessage>,
    pending_requests: &Arc<Mutex<std::collections::HashMap<u64, PendingRequestKind>>>,
    analyzer_state: &Arc<(Mutex<AnalyzerState>, Condvar)>,
) {
    if let Some(method) = message
        .get("method")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
    {
        handle_server_message(message, &method, tx_out, analyzer_state);
        return;
    }

    let Some(id) = message.get("id") else {
        return;
    };
    let Some(request_id) = id.as_u64() else {
        return;
    };
    if request_id == 1 {
        let _ = tx.send(LspEvent::InitializeComplete);
        return;
    }
    let request_kind = pending_requests
        .lock()
        .ok()
        .and_then(|mut pending| pending.remove(&request_id));
    if let Some(result) = message.get("result") {
        match request_kind {
            Some(PendingRequestKind::Hover) => {
                let contents = hover_contents_to_string(result.get("contents"));
                let _ = tx.send(LspEvent::HoverResponse {
                    request_id,
                    contents,
                });
            }
            Some(PendingRequestKind::Completion) | None => {
                let items = parse_completion_items(result);
                if tx
                    .send(LspEvent::CompletionResponse { request_id, items })
                    .is_err()
                {
                    warn!("failed to send completion response to editor loop");
                }
            }
        }
    } else {
        debug!("LSP response without result for id={request_id}");
    }
}

fn handle_server_message(
    message: Value,
    method: &str,
    tx_out: &Sender<OutboundMessage>,
    analyzer_state: &Arc<(Mutex<AnalyzerState>, Condvar)>,
) {
    match method {
        "window/workDoneProgress/create" => {
            if let Some(id) = message.get("id").and_then(Value::as_u64) {
                let _ = tx_out.send(OutboundMessage::Json(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": Value::Null,
                })));
            }
        }
        "$/progress" => update_progress_state(message.get("params"), analyzer_state),
        "experimental/serverStatus" => {
            update_server_status(message.get("params"), analyzer_state)
        }
        _ => {}
    }
}

fn update_progress_state(
    params: Option<&Value>,
    analyzer_state: &Arc<(Mutex<AnalyzerState>, Condvar)>,
) {
    let Some(kind) = params
        .and_then(|params| params.get("value"))
        .and_then(|value| value.get("kind"))
        .and_then(Value::as_str)
    else {
        return;
    };

    let (lock, cvar) = &**analyzer_state;
    let Ok(mut state) = lock.lock() else {
        return;
    };
    state.progress_seen = true;

    match kind {
        "begin" => {
            state.pending_work = state.pending_work.saturating_add(1);
            state.ready = false;
        }
        "report" => {
            if state.pending_work > 0 {
                state.ready = false;
            }
        }
        "end" => {
            state.pending_work = state.pending_work.saturating_sub(1);
            if state.pending_work == 0 {
                state.ready = true;
            }
        }
        _ => {}
    }

    cvar.notify_all();
}

fn update_server_status(
    params: Option<&Value>,
    analyzer_state: &Arc<(Mutex<AnalyzerState>, Condvar)>,
) {
    let Some(quiescent) = params.and_then(|params| params.get("quiescent")).and_then(Value::as_bool)
    else {
        return;
    };

    let (lock, cvar) = &**analyzer_state;
    let Ok(mut state) = lock.lock() else {
        return;
    };
    state.server_status_seen = true;
    state.ready = quiescent;
    if quiescent {
        state.pending_work = 0;
    }
    cvar.notify_all();
}

fn parse_completion_items(result: &Value) -> Vec<CompletionItem> {
    let raw_items = if result.is_array() {
        result.as_array().cloned().unwrap_or_default()
    } else {
        result
            .get("items")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
    };

    raw_items
        .into_iter()
        .filter_map(|item| {
            let label = item.get("label").and_then(Value::as_str)?.to_string();
            let insert_text = item
                .get("insertText")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
                .or_else(|| {
                    item.get("textEdit")
                        .and_then(Value::as_object)
                        .and_then(|obj| obj.get("newText"))
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned)
                })
                .unwrap_or_else(|| label.clone());

            Some(CompletionItem { label, insert_text })
        })
        .collect()
}

fn path_to_file_uri(path: &Path) -> io::Result<String> {
    let absolute = absolute_path(path)?;

    let mut as_str = absolute.to_string_lossy().replace('\\', "/");
    if as_str.len() >= 2 && as_str.as_bytes()[1] == b':' {
        as_str = format!("/{as_str}");
    }

    Ok(format!("file://{as_str}"))
}

fn absolute_path(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn choose_launch_spec() -> io::Result<LaunchSpec> {
    let mut candidates: Vec<LaunchSpec> = vec![
        LaunchSpec {
            program: "rust-analyzer".to_string(),
            args: Vec::new(),
        },
        LaunchSpec {
            program: "rust-analyzer.exe".to_string(),
            args: Vec::new(),
        },
        LaunchSpec {
            program: "rustup".to_string(),
            args: vec!["run".to_string(), "stable".to_string(), "rust-analyzer".to_string()],
        },
    ];

    if let Some(path) = cargo_home_rust_analyzer_path() {
        candidates.insert(
            0,
            LaunchSpec {
                program: path,
                args: Vec::new(),
            },
        );
    }

    for candidate in candidates {
        if command_supports_version(&candidate) {
            return Ok(candidate);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "rust-analyzer is unavailable. Install with: rustup component add rust-analyzer",
    ))
}

fn command_supports_version(spec: &LaunchSpec) -> bool {
    let mut cmd = Command::new(&spec.program);
    cmd.args(&spec.args).arg("--version");
    match cmd
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn format_launch_spec(spec: &LaunchSpec) -> String {
    if spec.args.is_empty() {
        spec.program.clone()
    } else {
        format!("{} {}", spec.program, spec.args.join(" "))
    }
}

fn cargo_home_rust_analyzer_path() -> Option<String> {
    let base = std::env::var("CARGO_HOME").ok().or_else(|| {
        std::env::var("USERPROFILE")
            .ok()
            .map(|home| format!("{home}\\.cargo"))
    })?;

    let mut path = PathBuf::from(base);
    path.push("bin");
    #[cfg(windows)]
    {
        path.push("rust-analyzer.exe");
    }
    #[cfg(not(windows))]
    {
        path.push("rust-analyzer");
    }
    if path.exists() {
        Some(path.to_string_lossy().to_string())
    } else {
        None
    }
}

fn detect_workspace_root(file_path: &Path) -> io::Result<PathBuf> {
    let mut dir = if file_path.is_dir() {
        file_path.to_path_buf()
    } else {
        file_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    };

    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }

    if file_path.is_file() {
        Ok(file_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from(".")))
    } else {
        Ok(file_path.to_path_buf())
    }
}

fn is_hover_result(result: &Value) -> bool {
    result.get("contents").is_some() || result.is_null()
}

fn hover_contents_to_string(contents: Option<&Value>) -> Option<String> {
    let contents = contents?;
    match contents {
        Value::String(s) => Some(s.clone()),
        Value::Array(items) => {
            let parts = items
                .iter()
                .filter_map(hover_marked_string_to_text)
                .collect::<Vec<_>>();
            if parts.is_empty() {
                None
            } else {
                Some(parts.join("\n---\n"))
            }
        }
        Value::Object(_) => hover_marked_string_to_text(contents),
        _ => None,
    }
}

fn hover_marked_string_to_text(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Object(map) => {
            if let Some(text) = map.get("value").and_then(Value::as_str) {
                Some(text.to_string())
            } else {
                map.get("language")
                    .and_then(Value::as_str)
                    .zip(map.get("value").and_then(Value::as_str))
                    .map(|(lang, val)| format!("```{lang}\n{val}\n```"))
            }
        }
        _ => None,
    }
}