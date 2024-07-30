/// Zero-based offset of bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset(usize);

impl Offset {
    pub fn new(raw: usize) -> Self {
        Self(raw)
    }

    pub fn raw(&self) -> usize {
        self.0
    }
}

impl From<usize> for Offset {
    fn from(value: usize) -> Self {
        Offset::new(value)
    }
}

/// Zero-based (line, column) location
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineIndex {
    line: usize,
    column: usize,
}

impl LineIndex {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn raw(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    /// Get one-based line and column numbers
    pub fn one_based(&self) -> (usize, usize) {
        (self.line + 1, self.column + 1)
    }
}

impl From<(usize, usize)> for LineIndex {
    fn from(value: (usize, usize)) -> Self {
        LineIndex {
            line: value.0,
            column: value.1,
        }
    }
}
