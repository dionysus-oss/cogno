use std::fmt::Debug;
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
            panic_info: None,
            completed: false,
            assertions: Vec::new(),
        });
    }

    pub fn set_panic_info(&mut self, info: String) {
        self.tests.last_mut().unwrap().panic_info = Some(info);
    }

    pub fn complete(&mut self) {
        self.tests.last_mut().unwrap().completed = true;
    }

    pub fn must_eq<T: PartialEq + Debug>(&mut self, id: &str, expected: T, actual: T) {
        self.append_assert(id, AssertionType::Must, expected, actual);
    }

    pub fn must_not_eq<T: PartialEq + Debug>(&mut self, id: &str, expected: T, actual: T) {
        self.append_assert(id, AssertionType::MustNot, expected, actual);
    }

    pub fn should_eq<T: PartialEq + Debug>(&mut self, id: &str, expected: T, actual: T) {
        self.append_assert(id, AssertionType::Should, expected, actual);
    }

    pub fn should_not_eq<T: PartialEq + Debug>(&mut self, id: &str, expected: T, actual: T) {
        self.append_assert(id, AssertionType::ShouldNot, expected, actual);
    }

    pub fn may_eq<T: PartialEq + Debug>(&mut self, id: &str, expected: T, actual: T) {
        self.append_assert(id, AssertionType::May, expected, actual);
    }

    fn append_assert<T: PartialEq + Debug>(&mut self, id: &str, kind: AssertionType, expected: T, actual: T) {
        let result = expected == actual;

        let error_message = if self.is_passed_assertion(&kind, result) {
            None
        } else {
            Some(format!("expected [{:?}] but was [{:?}]", expected, actual))
        };

        let def = AssertionDef {
            id: id.to_string(),
            kind,
            result,
            error_message,
        };

        self.tests.last_mut().unwrap().assertions.push(def);
    }

    fn is_passed_assertion(&self, kind: &AssertionType, result: bool) -> bool {
        match kind {
            AssertionType::Must => result,
            AssertionType::MustNot => !result,
            AssertionType::Should => result,
            AssertionType::ShouldNot => !result,
            AssertionType::May => result,
        }
    }
}

#[derive(Debug)]
struct TestDef {
    name: String,
    panic_info: Option<String>,
    completed: bool,
    assertions: Vec<AssertionDef>,
}

#[derive(Debug)]
struct AssertionDef {
    id: String,
    kind: AssertionType,
    result: bool,
    error_message: Option<String>,
}

#[derive(Debug)]
enum AssertionType {
    Must,
    MustNot,
    Should,
    ShouldNot,
    May,
}
