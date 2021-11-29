use std::path::Path;
use std::process::{Command};

use crate::WizError;

pub fn cmd<A: AsRef<str>, P: AsRef<Path>>(
    name: &str,
    args: &[A],
    dir: P,
    print: bool
) -> Result<String, WizError> {
    let mut cmd = Command::new(name);
    for arg in args {
        cmd.arg(arg.as_ref());
    }
    cmd.current_dir(dir);
    let child = cmd.output()?;
    let result = String::from_utf8(child.stdout)?;
    if print {
        println!("{}", result.trim());
    }
    Ok(result)
}

pub fn cp(origin: &str, destiny: &str) -> Result<(), WizError> {
    if std::fs::metadata(origin)?.is_dir() {
        copy_directory(origin, destiny)?;
    } else {
        copy_file(origin, destiny)?;
    }
    Ok(())
}

pub fn mv(origin: &str, destiny: &str) -> Result<(), WizError> {
    cp(origin, destiny)?;
    rm(origin)?;
    Ok(())
}

pub fn rm(path: &str) -> Result<(), WizError> {
    if std::fs::metadata(path)?.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

pub fn mk_dir(path: &str) -> Result<(), WizError> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn mk_file(path: &str, contents: &str) -> Result<(), WizError> {
    std::fs::write(path, contents)?;
    Ok(())
}

pub fn copy_directory(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), WizError> {
    std::fs::create_dir_all(&destiny)?;
    for entry in std::fs::read_dir(origin)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_directory(entry.path(), destiny.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destiny.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn copy_file(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), WizError> {
    if let Some(parent) = destiny.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(origin, destiny)?;
    Ok(())
}
