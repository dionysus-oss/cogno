use std::path::PathBuf;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ModuleRef {
    path: PathBuf,
    relative_path: PathBuf,
    test_functions: Vec<String>,
}

impl ModuleRef {
    pub fn new(path: PathBuf, relative_path: PathBuf) -> Self {
        ModuleRef {
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

}
