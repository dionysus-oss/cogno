pub use assert::*;
pub use attr::*;

#[derive(Debug)]
pub struct TestRecorder {
    tests: Vec<TestDef>,
}

impl TestRecorder {
    pub fn new() -> Self {
        TestRecorder { tests: Vec::new() }
    }

    pub fn register(&mut self, name: &str) {
        self.tests.push(TestDef {
            name: name.to_string(),
            completed: false,
        });
    }

    pub fn complete(&mut self) {
        self.tests.last_mut().unwrap().completed = true;
    }
}

#[derive(Debug)]
struct TestDef {
    name: String,
    completed: bool,
}
