use std::error::Error;
use std::marker::Send;
use std::marker::Sync;

use std::env;

mod locks;
mod repos;

pub type WizError = Box<dyn Error + Send + Sync>;

fn main() -> Result<(), WizError> {
	let mut clean = false;
	for arg in env::args() {
		if arg == "-v" || arg == "--version" {
			let version = env!("CARGO_PKG_VERSION");
			println!("QinpelWiz {}", version);
			return Ok(());
		} else if arg == "-c" || arg == "--clean" {
			clean = true;
		}
	}
	wizard(clean)?;
	Ok(())
}

fn wizard(clean: bool) -> Result<(), WizError> {
	println!("--- Starting Qinpel Wizard... ---");
	std::fs::create_dir_all("./code")?;
	std::fs::create_dir_all("./run")?;
	println!("Getting all Qinpel repositories...");
	let repos = repos::get_qinpel_repos()?;
	for repo in repos {
		println!("--- Starting Qinpel wizard of {} ---", repo.name);
		if let Err(e) = repo.wizard(clean) {
			eprintln!("Problem on wizard of {} - {}", repo.name, e);
		}
	}
	Ok(())
}
