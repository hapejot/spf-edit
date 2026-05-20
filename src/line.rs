//! Line data model.
//!
//! Each line in the buffer is a `Line` with:
//! - `line_type` — Data, sentinel (TopOfData/BottomOfData), ColsRuler, or Message.
//! - `data`      — The actual text content (empty for sentinels).
//! - `flags`     — Bitflags for state tracking (modified, pending cmd, etc.).
//! - `prefix_cmd` — Text the user typed into the prefix area (set on Enter,
//!                  cleared after command execution).
//!
//! Sentinel lines are non-editable markers; data lines hold file content.

// --- Line type ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    Data,
    TopOfData,
    BottomOfData,
    ColsRuler,
    Message,
}

// --- Line flags ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineFlags(u8);

impl LineFlags {
    pub const NONE: LineFlags = LineFlags(0);
    pub const MODIFIED: LineFlags = LineFlags(1 << 0);
    pub const PENDING_CMD: LineFlags = LineFlags(1 << 1);
    pub const EXCLUDED: LineFlags = LineFlags(1 << 2);
    pub const INSERTED: LineFlags = LineFlags(1 << 3);
    pub const CMD_ERROR: LineFlags = LineFlags(1 << 4);

    pub fn contains(self, other: LineFlags) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn set(&mut self, flag: LineFlags) {
        self.0 |= flag.0;
    }

    pub fn clear(&mut self, flag: LineFlags) {
        self.0 &= !flag.0;
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl std::ops::BitOr for LineFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        LineFlags(self.0 | rhs.0)
    }
}

// --- Line ---

#[derive(Debug, Clone)]
pub struct Line {
    pub line_type: LineType,
    pub data: String,
    pub original_number: Option<usize>,
    pub current_number: usize,
    pub flags: LineFlags,
    pub prefix_cmd: Option<String>,
    pub excluded: bool,
    pub label: Option<String>,
}

impl Line {
    pub fn new_data(data: String, number: usize) -> Self {
        Line {
            line_type: LineType::Data,
            data,
            original_number: Some(number),
            current_number: number,
            flags: LineFlags::NONE,
            prefix_cmd: None,
            excluded: false,
            label: None,
        }
    }

    pub fn new_blank(number: usize) -> Self {
        Line {
            line_type: LineType::Data,
            data: String::new(),
            original_number: None,
            current_number: number,
            flags: LineFlags::INSERTED,
            prefix_cmd: None,
            excluded: false,
            label: None,
        }
    }

    pub fn top_of_data() -> Self {
        Line {
            line_type: LineType::TopOfData,
            data: String::new(),
            original_number: None,
            current_number: 0,
            flags: LineFlags::NONE,
            prefix_cmd: None,
            excluded: false,
            label: None,
        }
    }

    pub fn bottom_of_data() -> Self {
        Line {
            line_type: LineType::BottomOfData,
            data: String::new(),
            original_number: None,
            current_number: 0,
            flags: LineFlags::NONE,
            prefix_cmd: None,
            excluded: false,
            label: None,
        }
    }

    pub fn cols_ruler() -> Self {
        Line {
            line_type: LineType::ColsRuler,
            data: String::new(),
            original_number: None,
            current_number: 0,
            flags: LineFlags::NONE,
            prefix_cmd: None,
            excluded: false,
            label: None,
        }
    }

    pub fn message(text: String) -> Self {
        Line {
            line_type: LineType::Message,
            data: text,
            original_number: None,
            current_number: 0,
            flags: LineFlags::NONE,
            prefix_cmd: None,
            excluded: false,
            label: None,
        }
    }

    pub fn is_data(&self) -> bool {
        self.line_type == LineType::Data
    }

    pub fn is_sentinel(&self) -> bool {
        matches!(self.line_type, LineType::TopOfData | LineType::BottomOfData)
    }

    pub fn is_writable(&self) -> bool {
        self.line_type == LineType::Data
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn clear_prefix_cmd(&mut self) {
        self.prefix_cmd = None;
        self.flags.clear(LineFlags::PENDING_CMD);
        self.flags.clear(LineFlags::CMD_ERROR);
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, LineType};

    fn text_to_lines(text: &str) -> Vec<Line> {
        text.split('\n')
            .enumerate()
            .map(|(idx, s)| Line::new_data(s.to_string(), idx + 1))
            .collect()
    }

    fn lines_to_text(lines: &[Line]) -> String {
        lines
            .iter()
            .filter(|line| line.line_type == LineType::Data)
            .map(|line| line.data.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn text_to_lines_creates_data_lines_with_numbers() {
        let lines = text_to_lines("alpha\nbeta\ngamma");

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].line_type, LineType::Data);
        assert_eq!(lines[0].data, "alpha");
        assert_eq!(lines[0].current_number, 1);
        assert_eq!(lines[1].data, "beta");
        assert_eq!(lines[1].current_number, 2);
        assert_eq!(lines[2].data, "gamma");
        assert_eq!(lines[2].current_number, 3);
    }

    #[test]
    fn lines_to_text_ignores_non_data_line_types() {
        let lines = vec![
            Line::top_of_data(),
            Line::new_data("row-1".to_string(), 1),
            Line::cols_ruler(),
            Line::message("status".to_string()),
            Line::new_data("row-2".to_string(), 2),
            Line::bottom_of_data(),
        ];

        assert_eq!(lines_to_text(&lines), "row-1\nrow-2");
    }

    #[test]
    fn text_round_trip_preserves_short_sample() {
        let original = "one\n\nthree";
        let lines = text_to_lines(original);
        let rebuilt = lines_to_text(&lines);

        assert_eq!(rebuilt, original);
    }

    #[test]
    fn line_type_helpers_match_expected_behavior() {
        let data = Line::new_data("x".to_string(), 1);
        let top = Line::top_of_data();
        let bottom = Line::bottom_of_data();
        let cols = Line::cols_ruler();
        let msg = Line::message("note".to_string());

        assert!(data.is_data());
        assert!(data.is_writable());
        assert!(!data.is_sentinel());

        assert!(top.is_sentinel());
        assert!(!top.is_writable());
        assert!(bottom.is_sentinel());
        assert!(!bottom.is_writable());

        assert!(!cols.is_sentinel());
        assert!(!cols.is_writable());
        assert!(!msg.is_sentinel());
        assert!(!msg.is_writable());
    }
}
