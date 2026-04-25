//! FileBuffer: owns the line storage and file metadata.
//!
//! This is the "model" in the MVC-ish split.  It provides all structural
//! editing operations (insert, delete, copy, move, repeat, renumber, find)
//! and delegates persistence to `file_io`.
//!
//! ## Known Issues
//!
//! - `find_text` uses byte-based `data_lower[search_from..]` slicing which
//!   will panic if `search_from` isn't on a char boundary (e.g. if cursor
//!   was positioned mid-character via horizontal scrolling).  Should use
//!   char-index conversion.  TODO: fix for UTF-8 safety.
//! - `rebuild_label_indices` is a no-op stub — labels are lost after any
//!   structural edit (insert/delete/move).  A proper implementation should
//!   store the label on the `Line` itself so it survives renumbering.
//! - `move_lines` adjusts `dest_index` for removed lines, but doesn't
//!   account for sentinel lines between source and dest.  Edge cases with
//!   sentinels near move targets may misplace lines.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use tracing::{info, debug, trace};

use crate::line::{Line, LineFlags};
use crate::line_store::{LineStore, VecLineStore};
use crate::types::{
    Direction, LineEnding, NumberMode, RecordFormat, LINE_NUMBER_INCREMENT,
};
use crate::file_io;

pub struct FileBuffer {
    pub lines: VecLineStore,
    pub file_path: PathBuf,
    pub modified: bool,
    pub record_format: RecordFormat,
    pub line_ending: LineEnding,
    pub caps_mode: bool,
    pub number_mode: NumberMode,
    pub nulls_mode: bool,
    pub bounds: (usize, usize), // (left, right) 1-based column bounds, 0 = unbounded
    pub labels: HashMap<String, usize>,
    pub browse_mode: bool,
}

impl FileBuffer {
    /// Open a file and create a buffer.
    pub fn open(path: &Path, record_format: RecordFormat, browse: bool) -> io::Result<Self> {
        info!("FileBuffer::open {:?} format={:?} browse={}", path, record_format, browse);
        let (lines, line_ending) = file_io::read_file(path, record_format)?;
        info!("  loaded {} lines, line_ending={:?}", lines.iter().count(), line_ending);

        Ok(FileBuffer {
            lines,
            file_path: path.to_path_buf(),
            modified: false,
            record_format,
            line_ending,
            caps_mode: false,
            number_mode: NumberMode::On,
            nulls_mode: true,
            bounds: (0, 0),
            labels: HashMap::new(),
            browse_mode: browse,
        })
    }

    /// Create a buffer for a new (non-existent) file.
    pub fn new_empty(path: &Path, record_format: RecordFormat) -> Self {
        FileBuffer {
            lines: file_io::create_empty_buffer(),
            file_path: path.to_path_buf(),
            modified: false,
            record_format,
            line_ending: LineEnding::Lf,
            caps_mode: false,
            number_mode: NumberMode::On,
            nulls_mode: true,
            bounds: (0, 0),
            labels: HashMap::new(),
            browse_mode: false,
        }
    }

    /// Save the buffer to disk.
    pub fn save(&mut self) -> io::Result<()> {
        info!("FileBuffer::save to {:?}", self.file_path);
        file_io::write_file(
            &self.file_path,
            &self.lines,
            self.record_format,
            self.line_ending,
            self.nulls_mode,
        )?;
        self.modified = false;

        // Clear modified flags on all lines
        for i in 0..self.lines.len() {
            if let Some(line) = self.lines.get_mut(i) {
                line.flags.clear(LineFlags::MODIFIED);
            }
        }

        Ok(())
    }

    /// Total number of lines (including sentinels).
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Count of data lines only.
    pub fn data_line_count(&self) -> usize {
        (0..self.lines.len())
            .filter(|&i| {
                self.lines
                    .get(i)
                    .map(|l| l.is_data())
                    .unwrap_or(false)
            })
            .count()
    }

    // --- Line operations ---

    /// Insert `count` blank lines after `index`.
    pub fn insert_lines_after(&mut self, index: usize, count: usize) {
        debug!("insert_lines_after: index={index} count={count}");
        let number = self
            .lines
            .get(index)
            .map(|l| l.current_number)
            .unwrap_or(0);

        for i in 0..count {
            let line = Line::new_blank(number + i + 1);
            self.lines.insert(index + 1 + i, line);
        }
        self.modified = true;
        self.renumber();
    }

    /// Delete `count` lines starting at `index`. Skips sentinels.
    pub fn delete_lines(&mut self, index: usize, count: usize) {
        debug!("delete_lines: index={index} count={count}");
        let mut deleted = 0;
        let mut pos = index;
        while deleted < count && pos < self.lines.len() {
            if let Some(line) = self.lines.get(pos) {
                if line.is_sentinel() {
                    pos += 1;
                    continue;
                }
            }
            self.lines.remove(pos);
            deleted += 1;
        }
        if deleted > 0 {
            self.modified = true;
            self.renumber();
        }
    }

    /// Delete all lines in the range [start, end] inclusive. Skips sentinels.
    pub fn delete_range(&mut self, start: usize, end: usize) {
        let count = end.saturating_sub(start) + 1;
        self.delete_lines(start, count);
    }

    /// Copy lines from [src_start, src_end] inclusive and insert after dest_index.
    pub fn copy_lines(&mut self, src_start: usize, src_end: usize, dest_index: usize) {
        debug!("copy_lines: [{src_start}..{src_end}] -> after {dest_index}");
        let mut copies = Vec::new();
        for i in src_start..=src_end {
            if let Some(line) = self.lines.get(i) {
                if line.is_data() {
                    let mut copy = line.clone();
                    copy.original_number = None;
                    copy.flags = LineFlags::INSERTED;
                    copy.prefix_cmd = None;
                    copies.push(copy);
                }
            }
        }
        if !copies.is_empty() {
            self.lines.insert_many(dest_index + 1, copies);
            self.modified = true;
            self.renumber();
        }
    }

    /// Move lines from [src_start, src_end] inclusive to after dest_index.
    pub fn move_lines(&mut self, src_start: usize, src_end: usize, dest_index: usize) {
        debug!("move_lines: [{src_start}..{src_end}] -> after {dest_index}");
        // Drain source lines (only data lines)
        let mut moved: Vec<Line> = Vec::new();
        let mut i = src_start;
        while i <= src_end && i < self.lines.len() {
            if let Some(line) = self.lines.get(i) {
                if line.is_data() {
                    let mut line = self.lines.remove(i);
                    line.flags.set(LineFlags::MODIFIED);
                    line.prefix_cmd = None;
                    moved.push(line);
                    // After remove, the end shifts down, but we keep i the same
                    // since the next element slides into position i
                    continue;
                }
            }
            i += 1;
        }

        if !moved.is_empty() {
            // Adjust dest_index if it was after the removed range
            let actual_dest = if dest_index > src_start {
                dest_index - moved.len()
            } else {
                dest_index
            };
            self.lines.insert_many(actual_dest + 1, moved);
            self.modified = true;
            self.renumber();
        }
    }

    /// Repeat (duplicate) lines from [start, end] inclusive, `times` copies.
    pub fn repeat_lines(&mut self, start: usize, end: usize, times: usize) {
        debug!("repeat_lines: [{start}..{end}] x{times}");
        let mut source = Vec::new();
        for i in start..=end {
            if let Some(line) = self.lines.get(i) {
                if line.is_data() {
                    let mut copy = line.clone();
                    copy.original_number = None;
                    copy.flags = LineFlags::INSERTED;
                    copy.prefix_cmd = None;
                    source.push(copy);
                }
            }
        }

        if !source.is_empty() {
            let insert_at = end + 1;
            let mut all_copies = Vec::new();
            for _ in 0..times {
                all_copies.extend(source.clone());
            }
            self.lines.insert_many(insert_at, all_copies);
            self.modified = true;
            self.renumber();
        }
    }

    /// Renumber all data lines based on number_mode.
    pub fn renumber(&mut self) {
        let mut num = 0usize;
        for i in 0..self.lines.len() {
            if let Some(line) = self.lines.get_mut(i) {
                if line.is_data() {
                    num += LINE_NUMBER_INCREMENT;
                    line.current_number = num;
                }
            }
        }
        // Update labels
        self.rebuild_label_indices();
    }

    /// Set a label on a line.
    pub fn set_label(&mut self, name: String, index: usize) {
        self.labels.insert(name, index);
    }

    /// Get the line index for a label.
    pub fn get_label(&self, name: &str) -> Option<usize> {
        // Handle system labels
        match name.to_uppercase().as_str() {
            ".ZFIRST" => {
                for i in 0..self.lines.len() {
                    if let Some(l) = self.lines.get(i) {
                        if l.is_data() {
                            return Some(i);
                        }
                    }
                }
                None
            }
            ".ZLAST" => {
                for i in (0..self.lines.len()).rev() {
                    if let Some(l) = self.lines.get(i) {
                        if l.is_data() {
                            return Some(i);
                        }
                    }
                }
                None
            }
            _ => self.labels.get(name).copied(),
        }
    }

    /// Rebuild label index mapping after lines have moved.
    fn rebuild_label_indices(&mut self) {
        // For now, we don't persist label-to-line mappings across renumbers.
        // Labels are cleared on structural changes. A future improvement could
        // store labels on the Line struct itself.
        // We keep existing labels valid only until the next structural edit.
    }

    /// Find text in the buffer.
    pub fn find_text(
        &self,
        query: &str,
        start_line: usize,
        start_col: usize,
        direction: Direction,
    ) -> Option<(usize, usize)> {
        debug!("find_text: {:?} from ({start_line},{start_col}) dir={:?}", query, direction);
        let query_lower = query.to_lowercase();

        match direction {
            Direction::Next => {
                // Search from start_line/start_col forward
                for i in start_line..self.lines.len() {
                    if let Some(line) = self.lines.get(i) {
                        if !line.is_data() {
                            continue;
                        }
                        let data_lower = line.data.to_lowercase();
                        let search_from = if i == start_line { start_col } else { 0 };
                        if let Some(pos) = data_lower[search_from..].find(&query_lower) {
                            return Some((i, search_from + pos));
                        }
                    }
                }
                None
            }
            Direction::Prev => {
                for i in (0..=start_line).rev() {
                    if let Some(line) = self.lines.get(i) {
                        if !line.is_data() {
                            continue;
                        }
                        let data_lower = line.data.to_lowercase();
                        let search_until = if i == start_line {
                            start_col
                        } else {
                            data_lower.len()
                        };
                        if let Some(pos) = data_lower[..search_until].rfind(&query_lower) {
                            return Some((i, pos));
                        }
                    }
                }
                None
            }
            Direction::First => {
                for i in 0..self.lines.len() {
                    if let Some(line) = self.lines.get(i) {
                        if !line.is_data() {
                            continue;
                        }
                        let data_lower = line.data.to_lowercase();
                        if let Some(pos) = data_lower.find(&query_lower) {
                            return Some((i, pos));
                        }
                    }
                }
                None
            }
            Direction::Last => {
                for i in (0..self.lines.len()).rev() {
                    if let Some(line) = self.lines.get(i) {
                        if !line.is_data() {
                            continue;
                        }
                        let data_lower = line.data.to_lowercase();
                        if let Some(pos) = data_lower.rfind(&query_lower) {
                            return Some((i, pos));
                        }
                    }
                }
                None
            }
            Direction::All => {
                // For ALL, return the first match; caller counts all
                self.find_text(query, 0, 0, Direction::First)
            }
        }
    }

    /// Count all occurrences of a string in data lines.
    pub fn count_occurrences(&self, query: &str) -> usize {
        let query_lower = query.to_lowercase();
        let mut count = 0;
        for i in 0..self.lines.len() {
            if let Some(line) = self.lines.get(i) {
                if !line.is_data() {
                    continue;
                }
                let data_lower = line.data.to_lowercase();
                count += data_lower.matches(&query_lower).count();
            }
        }
        count
    }

    /// Clear all pending line commands and errors.
    pub fn reset_commands(&mut self) {
        for i in 0..self.lines.len() {
            if let Some(line) = self.lines.get_mut(i) {
                line.clear_prefix_cmd();
            }
        }
    }

    /// Clear all labels.
    pub fn reset_labels(&mut self) {
        self.labels.clear();
    }

    /// Update a line's data content (from overtype editing).
    pub fn update_line_data(&mut self, index: usize, new_data: String) {
        if let Some(line) = self.lines.get_mut(index) {
            if line.is_data() && line.data != new_data {
                trace!("update_line_data: line {index} changed");
                line.data = new_data;
                line.flags.set(LineFlags::MODIFIED);
                self.modified = true;
            }
        }
    }

    /// Get the filename as a display string.
    pub fn display_name(&self) -> String {
        self.file_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| self.file_path.to_string_lossy().to_string())
    }
}
