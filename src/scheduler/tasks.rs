
pub struct Task {
    pub runner: Box<dyn Fn() -> Result<bool, String>>,
    pub delay: u32,
    pub block: bool,
}

unsafe impl Sync for Task {}

impl Task {
    pub fn run(&self) {
        (self.runner)();
    }

    pub fn new(f: &dyn Fn() -> Result<bool, String>, block: bool) -> Self {
        Task { 
            runner: Box::new(f),
            delay: 0,
            block
        }
    }

    pub fn new_with_delay(f: &dyn Fn() -> Result<bool, String>, block: bool, delay: u32) -> Self {
        Task { 
            runner: Box::new(f),
            delay,
            block
        }
    }
}