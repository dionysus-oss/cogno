use crate::error::CognoError;
use crate::report::model::AssertionType;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct SpecModifiers {
    pub spec_modifiers: Vec<SpecModifier>,
}

#[derive(Debug, Deserialize)]
pub struct SpecModifier {
    pub spec_id: String,
    pub test_modifiers: Vec<TestModifier>,
}

#[derive(Debug, Deserialize)]
pub struct TestModifier {
    pub test_id: String,
    pub assertion_modifiers: Vec<AssertionModifier>,
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct AssertionModifier {
    pub assertion_id: String,
    pub assertion_type: AssertionType,
}

pub fn load_spec_modifier(spec_modifier_path: &str) -> Result<SpecModifiers, CognoError> {
    let mut f = File::open(spec_modifier_path)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    toml::from_str(&buf).map_err(|e| e.into())
}
