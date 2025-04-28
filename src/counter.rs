struct Counter {
    val: usize,
}


impl Counter {
    #[unsafe(export_name="counter_new")]
    pub extern "C" fn new() -> Box<Self> {
        Box::new(Self {
            val: 0,
        })
    }

    #[unsafe(export_name="counter_get")]
    pub extern "C" fn get(&self) -> usize {
        self.val
    }

    #[unsafe(export_name="counter_increment")]
    pub extern "C" fn increment(&mut self) {
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
