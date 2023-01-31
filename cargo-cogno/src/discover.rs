use anyhow::Result;
use core::ModuleRef;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn discover<P: AsRef<Path>>(path: P) -> Result<Vec<ModuleRef>> {
    let mut source_files = discover_source_files(&path, &PathBuf::new())?;

    let pattern = Regex::new(r"(?sU)#\[cogno_test].*fn (?P<fname>.*)\(")?;

    for module_ref in source_files.iter_mut() {
        let mut file = File::open(module_ref.get_path())?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;

        for captures in pattern.captures_iter(string.as_str()) {
            if let Some(m) = captures.name("fname") {
                module_ref.add_function(m.as_str().to_string());
            }
        }
    }

    Ok(source_files
        .iter()
        .filter(|sf| sf.has_test_functions())
        .cloned()
        .collect())
}

fn discover_source_files<P1: AsRef<Path>>(path: &P1, sub: &PathBuf) -> Result<Vec<ModuleRef>> {
    let search = path.as_ref().join(sub);

    let mut output = Vec::new();

    let mut dir = search.read_dir()?;
    while let Some(Ok(s)) = dir.next() {
        let file_type = s.file_type()?;
        if file_type.is_dir() {
            let mut sub_found = discover_source_files(path, &sub.join(s.path()))?;
            output.append(&mut sub_found);
        } else if file_type.is_file() {
            if let Some(Some("rs")) = s.path().extension().map(|s| s.to_str()) {
                output.push(ModuleRef::new(
                    s.path(),
                    sub.join(s.path().file_name().unwrap().to_str().unwrap())
                        .into(),
                ));
            }
        }
    }

    Ok(output)
}
