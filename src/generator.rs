pub struct MyGenerator {
    count: u32,
    max_count: u32,
    is_reset: bool,
}

impl MyGenerator {
    pub fn new(max_count: u32) -> MyGenerator {
        MyGenerator {
            count: 0,
            max_count,
            is_reset: true,
        }
    }
}

impl Iterator for MyGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {

        if self.count < self.max_count {
            self.count += 1;
            Some(self.count)
        } else {
            self.is_reset = true;
            self.count = 0;
            Some(self.count)
        }
    }
}

pub fn get_next_value(generator: &mut MyGenerator) -> Option<u32> {
    generator.next()
}
