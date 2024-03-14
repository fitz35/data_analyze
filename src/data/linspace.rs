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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace_creation_and_properties() {
        let linspace = Linspace::new(0.0, 10.0, 11);
        assert_eq!(linspace.start, 0.0);
        assert_eq!(linspace.end, 10.0);
        assert_eq!(linspace.step, 1.0);
        assert_eq!(linspace.len, 11);
    }

    #[test]
    fn test_linspace_index_of() {
        let linspace = Linspace::new(0.0, 10.0, 11);
        // Within range
        assert_eq!(linspace.index_of(&5.0), Some(5));
        // Start of the range
        assert_eq!(linspace.index_of(&0.0), Some(0));
        // End of the range
        assert_eq!(linspace.index_of(&10.0), Some(10));
        // Outside range
        assert_eq!(linspace.index_of(&-1.0), None);
        assert_eq!(linspace.index_of(&11.0), None);
    }

    #[test]
    fn test_linspace_index_of_intermediate_value() {
        let linspace = Linspace::new(0.0, 10.0, 11);
       
        assert_eq!(linspace.index_of(&0.5), Some(0));
        
        // Halfway between 5.0 and 6.0
        assert_eq!(linspace.index_of(&5.5), Some(5)); 

        // Very close to 9, testing precision
        assert_eq!(linspace.index_of(&8.9999), Some(8)); 

        // Testing a value that's slightly beyond the end but should round to the last index
        assert_eq!(linspace.index_of(&10.0001), None);
    }

    #[test]
    fn test_linspace_get_len() {
        let linspace = Linspace::new(-5.0, 5.0, 21);
        assert_eq!(linspace.get_len(), 21);
    }

    #[test]
    fn test_linspace_get_range() {
        let linspace = Linspace::new(1.0, 3.0, 3);
        let range = linspace.get_range();
        assert_eq!(range.start, 1.0);
        assert_eq!(range.end, 3.0);
    }
}
