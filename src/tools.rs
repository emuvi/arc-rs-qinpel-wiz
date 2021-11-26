use std::error::Error;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn command<P: AsRef<Path>>(
    name: &str,
    args: &[&str],
    dir: P,
) -> Result<String, Box<dyn Error>> {
    let mut cmd = Command::new(name);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.current_dir(dir);
    let child = cmd.stdin(Stdio::null()).stdout(Stdio::piped()).spawn()?;
    let mut result = String::new();
    child.stdout.unwrap().read_to_string(&mut result)?;
    Ok(result)
}
