#![feature(test)]
extern crate test;

pub mod arrays;
pub mod complex;
pub mod counter;
pub mod levenshtein;


#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}


#[no_mangle]
pub extern fn sqr(x: f64) -> f64 {
    x * x
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sqr() {
        assert_eq!(sqr(5.0), 25.0);
    }
}
