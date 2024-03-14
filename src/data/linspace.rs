use std::ops::Range;

/// A struct to represent a linspace array
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Linspace {
    start : f64,
    end : f64,
    step : f64,
    len : usize,
}

impl Linspace {
    /// Create a new linspace array
    pub fn new(start : f64, end : f64, len : usize) -> Self {
        let step = (end - start) / (len - 1) as f64;
        Self {
            start,
            end,
            step,
            len,
        }
    }

    /// Get the index of a value in the linspace array
    pub fn index_of(&self, value : &f64) -> Option<usize> {
        if *value < self.start || *value > self.end {
            return None;
        }
        let index = ((value - self.start) / self.step) as usize;
        Some(index)
    }

    /// get the length of the linspace array
    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_range(&self) -> Range<f64> {
        self.start..self.end
    }
}