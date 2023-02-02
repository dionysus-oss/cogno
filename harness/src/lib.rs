use std::fmt::Debug;
pub use assert::*;
pub use attr::*;
use crate::report::model::{AssertionDef, AssertionType, is_a_not_assertion, is_passed_assertion, TestDef};
use crate::report::{ConsoleReporter, Reporter};

mod report;
mod spec;

#[derive(Debug)]
pub struct TestController {
    tests: Vec<TestDef>,
    reporter: Box<dyn Reporter>,
}

impl TestController {
    pub fn new() -> Self {
        TestController {
            tests: Vec::new(),
            reporter: create_reporter(),
        }
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
        let current_test = self.tests.last_mut().unwrap();
        current_test.panic_info = Some(info);
        self.reporter.report(current_test);
    }

    pub fn complete(&mut self) {
        let current_test = self.tests.last_mut().unwrap();
        current_test.completed = true;
        self.reporter.report(current_test);
    }

    pub fn finalize(&self) {
        self.reporter.finalize();
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

        let error_message = if is_passed_assertion(&kind, result) {
            None
        } else if is_a_not_assertion(&kind) {
            Some(format!("got [{:?}]", actual))
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
}

fn create_reporter() -> Box<dyn Reporter> {
    match std::env::var("COGNO_REPORTER").unwrap_or("".to_string()).as_str() {
        "console" | _ => {
            Box::new(ConsoleReporter::new())
        }
    }
}
