struct Counter {
    val: usize,
}


impl Counter {
    #[no_mangle]
    #[export_name="counter_new"]
    pub extern fn new() -> Box<Self> {
        Box::new(Self {
            val: 0,
        })
    }

    #[no_mangle]
    #[export_name="counter_get"]
    pub extern fn get(&self) -> usize {
        self.val
    }

    #[no_mangle]
    #[export_name="counter_increment"]
    pub extern fn increment(&mut self) {
        self.val += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut counter = Counter::new();
        assert_eq!(counter.get(), 0);
        counter.increment();
        assert_eq!(counter.get(), 1);
    }
}
