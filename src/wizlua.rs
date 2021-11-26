use rlua::{Lua, MultiValue};
use std::error::Error;
use std::path::Path;

pub fn execute<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let path_display = path.as_ref().to_str().unwrap();
    let source = std::fs::read_to_string(&path)?;
    let lua = Lua::new();
    lua.context(|ctx| match ctx.load(&source).eval::<MultiValue>() {
        Ok(values) => {
            let result = format!(
                "{}",
                values
                    .iter()
                    .map(|value| format!("{:?}", value))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            let result = result.trim();
            if result.is_empty() {
                println!("Successfully executed {} with no result.", path_display);
            } else {
                println!("Successfully executed {} with result(s): \n{}", path_display, result);
            }
        }
        Err(e) => {
            eprintln!("Error on execution of {} with message: \n{}", path_display, e);
        }
    });
    Ok(())
}
