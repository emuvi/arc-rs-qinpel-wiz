use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::WizError;

#[derive(Serialize, Deserialize)]
pub struct Locker {
    pub locked: HashMap<String, String>,
}

impl Locker {

    pub fn load() -> Result<Locker, WizError> {
        let path = std::path::Path::new("./locker.json");
        if path.exists() {
            let source = std::fs::read_to_string(path)?;
            let result: Locker = serde_json::from_str(&source)?;
            Ok(result)
        } else {
            Ok(Locker{
                locked: HashMap::new()
            })
        }
    }

    pub fn save(&self) -> Result<(), WizError> {
        let source = serde_json::to_string(&self)?;
        std::fs::write("./locker.json", source)?;
        Ok(())
    }

}
