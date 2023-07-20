use std::ptr;


#[no_mangle]
pub extern fn array_sum(size: usize, arr: *const f64) -> f64 {
    let mut res = 0.0;
    for idx in 0..size {
        unsafe {
            res += *arr.offset(idx as isize);
        }
    }
    res
}


#[no_mangle]
pub extern fn array_set(size: usize, arr: *mut f64, val: f64) {
    for idx in 0..size {
        unsafe {
            *arr.offset(idx as isize) = val;
        }
    }
}


#[no_mangle]
pub extern fn array3_zero(arr: &mut [f64; 3]) {
    for idx in 0..arr.len() {
        arr[idx] = 0.0;
    }
}


#[no_mangle]
pub extern fn array_concat(size1: usize, arr1: *const f64,
                           size2: usize, arr2: *const f64) -> *const f64 {
    let mut res = Vec::with_capacity(size1 + size2);
    res.resize(size1 + size2, 0.0);
    unsafe {
        ptr::copy(arr1, res.as_mut_ptr(), size1);
        ptr::copy(arr2, res.as_mut_ptr().add(size1), size2);
    }
    Box::new(res).as_ptr()
}


#[no_mangle]
pub extern fn array5_fill(val: f64) -> Box<[f64; 5]> {
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
    fn test_array_concat() {
        let arr1 = [1.0, 2.0];
        let arr2 = [3.0, 4.0, 5.0];
        let res = array_concat(arr1.len(), arr1.as_ptr(), 
                               arr2.len(), arr2.as_ptr());
        let mut buffer = [0.0; 5];
        unsafe {
            ptr::copy(res, buffer.as_mut_ptr(), 5);
        }
        assert_eq!(buffer, [1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_array5_fill() {
        let res = array5_fill(2.5);
        assert_eq!(res, Box::new([2.5; 5]));
    }
}
