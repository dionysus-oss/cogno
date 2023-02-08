use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDef {
    pub name: String,
    pub spec_id: String,
    pub panic_info: Option<String>,
    pub completed: bool,
    pub assertions: Vec<AssertionDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionDef {
    pub id: String,
    pub kind: AssertionType,
    pub result: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssertionType {
    Must,
    MustNot,
    Should,
    ShouldNot,
    May,
}

#[cfg(feature = "console")]
#[derive(Debug)]
pub enum TestOutcome<'a> {
    Errored(String),
    AssertionsFailed(Vec<&'a AssertionDef>),
    Passed,
}

#[cfg(not(feature = "console"))]
#[derive(Debug)]
pub enum TestOutcome {
}

pub fn is_passed_assertion(kind: &AssertionType, result: bool) -> bool {
    match kind {
        AssertionType::Must => result,
        AssertionType::MustNot => !result,
        AssertionType::Should => result,
        AssertionType::ShouldNot => !result,
        AssertionType::May => result,
    }
}

pub fn is_a_not_assertion(kind: &AssertionType) -> bool {
    match kind {
        AssertionType::MustNot | AssertionType::ShouldNot => true,
        _ => false,
    }
}

impl TestDef {
    #[cfg(feature = "console")]
    pub fn get_test_outcome(&self) -> TestOutcome {
        if let Some(msg) = self.panic_info.clone() {
            return TestOutcome::Errored(msg);
        }

        if !self.completed {
            return TestOutcome::Errored("Did not complete".to_string());
        };

        let failed_assertions: Vec<&AssertionDef> = self
            .assertions
            .iter()
            .filter(|ta| ta.is_failed_assertion())
            .collect();
        if !failed_assertions.is_empty() {
            return TestOutcome::AssertionsFailed(failed_assertions);
        }

        TestOutcome::Passed
    }
}

impl AssertionDef {
    #[cfg(feature = "console")]
    fn is_failed_assertion(&self) -> bool {
        !is_passed_assertion(&self.kind, self.result)
    }
}
