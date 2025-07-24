pub struct LTGenerator {
    count: usize,
    max_count: usize,
    is_reset: bool,
    data: Vec<String>
}

impl LTGenerator {
    pub fn new(data: Vec<String>) -> LTGenerator {
        LTGenerator {
            count: 0,
            max_count: data.len() as usize,
            is_reset: true,
            data: data
        }
    }
}

impl Iterator for LTGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {

        if self.count < self.max_count {
            
            let value = self.data.get(self.count).cloned();
            self.count += 1;
            value
            
        } else {
            self.is_reset = true;
            self.count = 0;
            let value = self.data.get(self.count).cloned();
            value
        }
    }
}

pub fn get_next_value(generator: &mut LTGenerator) -> Option<String> {
    generator.next()
}
