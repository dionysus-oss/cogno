use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModuleRef {
    relative_path: PathBuf,
    test_functions: Vec<String>,
}

impl ModuleRef {
    pub fn to_source(&self) -> String {
        let parent =
            if self.relative_path.ends_with("main.rs") || self.relative_path.ends_with("lib.rs") {
                let parent = self.relative_path.parent();
                if let Some(p) = parent {
                    p
                } else {
                    self.relative_path.as_path()
                }
            } else {
                self.relative_path.as_path()
            }
                .with_extension("");

        let mut parent_import_path: String = itertools::intersperse(parent
                                                                  .iter()
                                                                  .map(|p| p.to_str().unwrap()), "::")
            .collect();
        if !parent_import_path.is_empty() {
            parent_import_path.push_str("::");
        }

        let mut out = String::new();
        for f in &self.test_functions {
            out.push_str(format!("{}{}(&mut controller);\n", parent_import_path, f).as_str());
        }

        out
    }
}