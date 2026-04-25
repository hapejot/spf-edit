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
