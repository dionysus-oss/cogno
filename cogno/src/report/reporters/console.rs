use crate::report::model::{AssertionType, TestDef, TestOutcome};
use crate::report::reporter::Reporter;
use colored::Colorize;
use crate::error::CognoError;

#[derive(Debug)]
pub struct ConsoleReporter {}

impl ConsoleReporter {
    pub fn new() -> Self {
        ConsoleReporter {}
    }
}

impl Reporter for ConsoleReporter {
    fn report(&mut self, test_def: &TestDef) {
        match test_def.get_test_outcome() {
            TestOutcome::Passed => {
                println!("{} - {}", "\u{2713}".green(), test_def.name.bold());
            }
            TestOutcome::Errored(msg) => {
                println!("{} - {}\n\t{}", "\u{2718}".red(), test_def.name.bold(), msg)
            }
            TestOutcome::AssertionsFailed(assertions) => {
                println!("{} - {}", "\u{2718}", test_def.name.bold());
                for assertion in assertions {
                    match assertion.kind {
                        AssertionType::Must | AssertionType::MustNot => {
                            println!(
                                "\t{} - {} {:?}: {}",
                                "\u{2718}".red(),
                                assertion.id,
                                assertion.kind,
                                assertion
                                    .error_message
                                    .as_ref()
                                    .unwrap_or(&"missing error message".to_string())
                            )
                        }
                        AssertionType::Should | AssertionType::ShouldNot => {
                            println!(
                                "\t{} - {} {:?}: {}",
                                "\u{2718}".blue(),
                                assertion.id,
                                assertion.kind,
                                assertion
                                    .error_message
                                    .as_ref()
                                    .unwrap_or(&"missing error message".to_string())
                            )
                        }
                        AssertionType::May => {
                            println!(
                                "\t{} - {} {:?}: {}",
                                "\u{2718}",
                                assertion.id,
                                assertion.kind,
                                assertion
                                    .error_message
                                    .as_ref()
                                    .unwrap_or(&"missing error message".to_string())
                            )
                        }
                    }
                }
            }
        }
    }

    fn finalize(&self) -> Result<(), CognoError> {
        // Nothing to do
        Ok(())
    }
}
