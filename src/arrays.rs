#[unsafe(no_mangle)]
pub extern "C" fn array_sum(size: usize, arr: *const f64) -> f64 {
    let mut res = 0.0;
    for idx in 0..size {
        unsafe {
            res += *arr.offset(idx as isize);
        }
    }
    res
}


#[unsafe(no_mangle)]
pub extern "C" fn array_set(size: usize, arr: *mut f64, val: f64) {
    for idx in 0..size {
        unsafe {
            *arr.offset(idx as isize) = val;
        }
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn array3_zero(arr: &mut [f64; 3]) {
    for idx in 0..arr.len() {
        arr[idx] = 0.0;
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn array5_fill(val: f64) -> Box<[f64; 5]> {
    Box::new([val; 5])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_sum() {
        let arr = [2.0, 4.0, 5.0];
        let res = array_sum(arr.len(), arr.as_ptr());
        assert_eq!(res, 11.0);
    }

    #[test]
    fn test_array_set() {
        let mut arr = [0.0; 5];
        array_set(arr.len(), arr.as_mut_ptr(), 3.0);
        assert_eq!(arr, [3.0; 5]);
    }

    #[test]
    fn test_array3_zero() {
        let mut arr = [3.0; 3];
        array3_zero(&mut arr);
        assert_eq!(arr, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_array5_fill() {
        let res = array5_fill(2.5);
        assert_eq!(res, Box::new([2.5; 5]));
    }
}
