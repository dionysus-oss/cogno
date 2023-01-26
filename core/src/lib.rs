// For intersperse in itertools vs stdlib
#![allow(unstable_name_collisions)]

use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRef {
    path: PathBuf,
    relative_path: PathBuf,
    test_functions: Vec<String>,
}

impl FunctionRef {
    pub fn new(path: PathBuf, relative_path: PathBuf) -> Self {
        FunctionRef {
            path,
            relative_path,
            test_functions: Vec::new(),
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn add_function(&mut self, f: String) {
        self.test_functions.push(f);
    }

    pub fn has_test_functions(&self) -> bool {
        !self.test_functions.is_empty()
    }

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

        let mut p: String = parent
            .iter()
            .map(|p| p.to_str().unwrap())
            .intersperse("::")
            .collect();
        if !p.is_empty() {
            p.push_str("::");
        }

        let mut out = String::new();
        for f in &self.test_functions {
            out.push_str(format!("{}{}(&mut recorder);\n", p, f).as_str());
        }

        out
    }
}

pub fn dump<P: AsRef<Path>>(fr: Vec<FunctionRef>, target: P) -> Result<()> {
    let serialised = serde_json::to_string(&fr)?;
    File::create(target)?.write_all(serialised.as_bytes())?;
    Ok(())
}

pub fn load<P: AsRef<Path>>(source: P) -> Result<Vec<FunctionRef>> {
    let mut content = String::new();
    File::open(source)?.read_to_string(&mut content)?;
    let out = serde_json::from_str(content.as_str())?;
    Ok(out)
}
