//! Line storage abstraction layer.
//!
//! All buffer operations go through the `LineStore` trait so the underlying
//! storage can be swapped (e.g. rope, mmap, lazy-loading) without changing
//! `FileBuffer` or `Editor`.  The MVP uses `VecLineStore` (a plain `Vec<Line>`).

use std::ops::Range;

use crate::line::Line;

/// Trait abstracting line storage. MVP uses VecLineStore; future implementations
/// can provide lazy-loading, rope-based, or remote storage without changing the
/// rest of the codebase.
pub trait LineStore {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, index: usize) -> Option<&Line>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Line>;

    fn insert(&mut self, index: usize, line: Line);
    fn remove(&mut self, index: usize) -> Line;

    /// Remove a range of lines and return them.
    fn drain(&mut self, range: Range<usize>) -> Vec<Line>;

    /// Replace a range of lines with a new set of lines.
    fn splice(&mut self, range: Range<usize>, replacement: Vec<Line>);

    /// Insert multiple lines at a given index.
    fn insert_many(&mut self, index: usize, lines: Vec<Line>) {
        // Default implementation: splice with empty range
        self.splice(index..index, lines);
    }
}

// --- VecLineStore ---

#[derive(Debug, Clone)]
pub struct VecLineStore {
    lines: Vec<Line>,
}

impl VecLineStore {
    pub fn new() -> Self {
        VecLineStore { lines: Vec::new() }
    }

    pub fn from_lines(lines: Vec<Line>) -> Self {
        VecLineStore { lines }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Line> {
        self.lines.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Line> {
        self.lines.iter_mut()
    }

    pub fn iter_range(&self, range: Range<usize>) -> std::slice::Iter<'_, Line> {
        let end = range.end.min(self.lines.len());
        let start = range.start.min(end);
        self.lines[start..end].iter()
    }
}

impl LineStore for VecLineStore {
    fn len(&self) -> usize {
        self.lines.len()
    }

    fn get(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Line> {
        self.lines.get_mut(index)
    }

    fn insert(&mut self, index: usize, line: Line) {
        self.lines.insert(index, line);
    }

    fn remove(&mut self, index: usize) -> Line {
        self.lines.remove(index)
    }

    fn drain(&mut self, range: Range<usize>) -> Vec<Line> {
        self.lines.drain(range).collect()
    }

    fn splice(&mut self, range: Range<usize>, replacement: Vec<Line>) {
        self.lines.splice(range, replacement);
    }
}

impl Default for VecLineStore {
    fn default() -> Self {
        Self::new()
    }
}
