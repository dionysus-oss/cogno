use crate::report::model::TestDef;
use std::fmt::Debug;

pub trait Reporter: Debug + Send {
    fn report(&mut self, test_def: &TestDef);

    fn finalize(&self);
}
