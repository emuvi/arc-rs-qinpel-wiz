use octocrab::Octocrab;
use std::error::Error;
use std::marker::Send;
use std::marker::Sync;

use std::env;

mod locks;
mod repos;

pub type WizError = Box<dyn Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), WizError> {
	for arg in env::args() {
		if arg == "-v" || arg == "--version" {
			let version = env!("CARGO_PKG_VERSION");
			println!("QinpelWiz {}", version);
			return Ok(());
		}
	}
	wizard().await?;
	Ok(())
}

async fn wizard() -> Result<(), WizError> {
	println!("--- Starting Qinpel Wizard... ---");
	std::fs::create_dir_all("./code")?;
	std::fs::create_dir_all("./run")?;
	let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
	let github = Octocrab::builder().personal_token(token).build()?;
	println!("Getting all Qinpel repositories...");
	let repos = repos::get_qinpel_repos(github).await?;
	for repo in repos {
		println!("--- Starting Qinpel wizard of {} ---", repo.name);
		if let Err(e) = repo.wizard().await {
			eprintln!("Problem on wizard of {} - {}", repo.name, e);
		}
	}
	Ok(())
}
