use crate::report::model::TestDef;
use std::fmt::Debug;
use crate::error::CognoError;

pub trait Reporter: Debug + Send {
    fn report(&mut self, test_def: &TestDef);

    fn finalize(&self) -> Result<(), CognoError>;
}
