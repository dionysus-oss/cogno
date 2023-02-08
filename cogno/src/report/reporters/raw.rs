use std::fs::File;
use std::io::Write;
use crate::error::CognoError;
use crate::report::model::TestDef;
use crate::report::Reporter;

#[derive(Debug)]
pub struct RawReporter {
    test_defs: Vec<TestDef>,
}

impl RawReporter {
    pub fn new() -> Self {
        RawReporter {
            test_defs: Vec::new(),
        }
    }
}

impl Reporter for RawReporter {
    fn report(&mut self, test_def: &TestDef) {
        self.test_defs.push(test_def.clone());
    }

    fn finalize(&self) -> Result<(), CognoError> {
        let str = serde_json::to_string(&self.test_defs)?;
        let mut f = File::create("cogno-report.json")?;
        f.write(str.as_bytes())?;

        Ok(())
    }
}
