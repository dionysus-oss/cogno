use std::fmt::Debug;
use crate::report::model::TestDef;

pub trait Reporter: Debug + Send {
    fn report(&mut self, test_def: &TestDef);

    fn finalize(&self);
}
