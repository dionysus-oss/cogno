use crate::error::CognoError;
use crate::report::model::{
    is_a_not_assertion, is_passed_assertion, AssertionDef, AssertionType, TestDef,
};
use crate::report::{ConsoleReporter, Reporter};
use crate::spec::{load_spec_modifier, AssertionModifier, SpecModifier};
pub use assert::*;
pub use attr::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Debug;
use crate::report::reporters::raw::RawReporter;

mod error;
mod report;
mod spec;

#[derive(Debug)]
pub struct TestController {
    tests: Vec<TestDef>,
    specs: HashSet<String>,
    modifiers: Vec<SpecModifier>,
    reporter: Box<dyn Reporter>,
}

impl TestController {
    pub fn new() -> Result<Self, CognoError> {
        let specs = load_specs();
        let modifiers = load_modifiers()?;

        Ok(TestController {
            tests: Vec::new(),
            specs,
            modifiers,
            reporter: create_reporter(),
        })
    }

    pub fn is_spec_enabled(&self, spec: &str) -> bool {
        self.specs.contains(spec)
    }

    pub fn register(&mut self, name: &str, spec_id: &str) {
        self.tests.push(TestDef {
            name: name.to_string(),
            spec_id: spec_id.to_string(),
            panic_info: None,
            completed: false,
            assertions: Vec::new(),
        });
    }

    pub fn set_panic_info(&mut self, info: String) {
        if self.tests.is_empty() {
            return;
        }

        let current_test = self.tests.last_mut().unwrap();
        current_test.panic_info = Some(info);
        self.reporter.report(current_test);
    }

    pub fn complete(&mut self) {
        let current_test = self.tests.last_mut().unwrap();
        current_test.completed = true;
        self.reporter.report(current_test);
    }

    pub fn finalize(&self) -> Result<(), CognoError> {
        self.reporter.finalize()
    }

    pub fn must_eq<T: PartialEq + Debug>(
        &mut self,
        id: &str,
        expected: T,
        actual: T,
    ) -> Result<(), CognoError> {
        self.append_assert(id, AssertionType::Must, expected, actual)
    }

    pub fn must_not_eq<T: PartialEq + Debug>(
        &mut self,
        id: &str,
        expected: T,
        actual: T,
    ) -> Result<(), CognoError> {
        self.append_assert(id, AssertionType::MustNot, expected, actual)
    }

    pub fn should_eq<T: PartialEq + Debug>(
        &mut self,
        id: &str,
        expected: T,
        actual: T,
    ) -> Result<(), CognoError> {
        self.append_assert(id, AssertionType::Should, expected, actual)
    }

    pub fn should_not_eq<T: PartialEq + Debug>(
        &mut self,
        id: &str,
        expected: T,
        actual: T,
    ) -> Result<(), CognoError> {
        self.append_assert(id, AssertionType::ShouldNot, expected, actual)
    }

    pub fn may_eq<T: PartialEq + Debug>(
        &mut self,
        id: &str,
        expected: T,
        actual: T,
    ) -> Result<(), CognoError> {
        self.append_assert(id, AssertionType::May, expected, actual)
    }

    fn append_assert<T: PartialEq + Debug>(
        &mut self,
        id: &str,
        kind: AssertionType,
        expected: T,
        actual: T,
    ) -> Result<(), CognoError> {
        let result = expected == actual;

        let error_message = if is_passed_assertion(&kind, result) {
            None
        } else if is_a_not_assertion(&kind) {
            Some(format!("got [{:?}]", actual))
        } else {
            Some(format!("expected [{:?}] but was [{:?}]", expected, actual))
        };

        let kind = self.assertion_or_override(id.to_string(), kind)?;

        let def = AssertionDef {
            id: id.to_string(),
            kind,
            result,
            error_message,
        };

        self.tests.last_mut().unwrap().assertions.push(def);

        Ok(())
    }

    fn assertion_or_override(
        &self,
        assertion_id: String,
        original_assertion_type: AssertionType,
    ) -> Result<AssertionType, CognoError> {
        let current_test = self.tests.last().unwrap();

        let matched_assertions: HashSet<&AssertionModifier> = self
            .modifiers
            .iter()
            .filter(|sm| sm.spec_id == current_test.spec_id)
            .flat_map(|sm| {
                sm.test_modifiers
                    .iter()
                    .filter(|tm| tm.test_id == current_test.name)
                    .flat_map(|tm| {
                        tm.assertion_modifiers
                            .iter()
                            .filter(|am| am.assertion_id == assertion_id)
                    })
            })
            .collect();

        if matched_assertions.len() > 1 {
            return Err(CognoError::ConflictingModifiers(format!(
                "{:?}",
                matched_assertions
            )));
        }

        Ok(match matched_assertions.iter().collect_vec().first() {
            Some(ma) => ma.assertion_type.clone(),
            None => original_assertion_type,
        })
    }
}

fn create_reporter() -> Box<dyn Reporter> {
    match std::env::var("COGNO_REPORTER")
        .unwrap_or("".to_string())
        .as_str()
    {
        "raw" => Box::new(RawReporter::new()),
        "console" | _ => Box::new(ConsoleReporter::new()),
    }
}

fn load_specs() -> HashSet<String> {
    std::env::var("COGNO_SPECS")
        .unwrap_or(String::new())
        .split(",")
        .map(|s| s.to_string())
        .collect()
}

fn load_modifiers() -> Result<Vec<SpecModifier>, CognoError> {
    let mut modifiers = Vec::new();
    for f in std::env::var("COGNO_MODIFIERS")
        .unwrap_or("".to_string())
        .split(",")
    {
        if f.is_empty() {
            continue;
        }

        modifiers.extend(load_spec_modifier(f)?.spec_modifiers);
    }

    Ok(modifiers)
}
