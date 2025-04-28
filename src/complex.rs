#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}


#[unsafe(no_mangle)]
pub extern "C" fn complex_len(z: Complex) -> f64 {
    (z.x * z.x + z.y * z.y).sqrt()
}


#[unsafe(no_mangle)]
pub extern "C" fn complex_conj(z: Complex) -> Complex {
    Complex {
        x: z.x,
        y: -z.y,
    }
}


impl Complex {
    #[unsafe(export_name="complex_real")]
    pub extern "C" fn real(&self) -> f64 {
        self.x
    }

    #[unsafe(export_name="complex_image")]
    pub extern "C" fn image(&self) -> f64 {
        self.y
    }

    #[unsafe(export_name="complex_mul")]
    pub extern "C" fn mul(&mut self, val: f64) {
        self.x *= val;
        self.y *= val;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_len() {
        let z = Complex { x: 3.0, y: -4.0 };
        assert_eq!(complex_len(z), 5.0);
    }

    #[test]
    fn test_complex_conj() {
        let z = Complex { x: 3.0, y: -4.0 };
        assert_eq!(complex_conj(z), Complex { x: 3.0, y: 4.0 });
    }

    #[test]
    fn test_complex_real() {
        let z = Complex { x: 3.0, y: -4.0 };
        assert_eq!(z.real(), 3.0);
    }

    #[test]
    fn test_complex_image() {
        let z = Complex { x: 3.0, y: -4.0 };
        assert_eq!(z.image(), -4.0);
    }

    #[test]
    fn test_complex_mul() {
        let mut z = Complex { x: 3.0, y: -4.0 };
        z.mul(2.0);
        assert_eq!(z, Complex { x: 6.0, y: -8.0 });
    }
}
