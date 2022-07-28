trait Example {
    fn count(&self) -> usize;
}

struct Something {
    size: usize,
}

impl Example for Something {
    fn count(&self) -> usize {
        self.size
    }
}

fn main() {
}