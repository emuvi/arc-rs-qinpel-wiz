use std::error::Error;
use std::marker::Send;
use std::marker::Sync;

use std::env;

mod locks;
mod repos;

pub type WizError = Box<dyn Error + Send + Sync>;

fn main() -> Result<(), WizError> {
	println!("--- Starting Qinpel Wizard... ---");
	println!("Creating all necessary folders...");
	std::fs::create_dir_all("./code")?;
	std::fs::create_dir_all("./run")?;
	let mut clean = false;
	let mut first = true;
	let mut passed_url = false;
	for arg in env::args() {
		if first {
			first = false;
			continue;
		}
		if arg == "-v" || arg == "--version" {
			println!("QinpelWiz {}", env!("CARGO_PKG_VERSION"));
			return Ok(());
		} else if arg == "-h" || arg == "--help" {
			print_help();
			return Ok(());
		} else if arg == "-c" || arg == "--clean" {
			clean = true;
		} else if arg.starts_with("https://") {
			passed_url = true;
			let repo = repos::Repository::new(&arg);
			println!("--- Starting Qinpel wizard of {} ---", repo.name);
			if let Err(e) = repo.wizard(clean) {
				eprintln!("Problem on wizard of {} - {}", repo.name, e);
			}
		}
	}
	if !passed_url {
		wizard_all(clean)?;
	}
	println!("--- Finished the Qinpel Wizard. ---");
	Ok(())
}

fn wizard_all(clean: bool) -> Result<(), WizError> {
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

fn print_help() {
	println!("QinpelWiz {}
Éverton M. Vieira <everton.muvi@gmail.com>
Qinpel Wizard - Transfer, compile and installs development bundles of graphical user and command line applications for the Qinpel (Quick Interface for Pointel) Platform.

USAGE:
    qinpel-wiz [FLAGS] [URL]...

FLAGS:
    -v, --version    Prints version information
    -h, --help       Prints help information
    -c, --clean      Removes the folder of each project before starts the wizard of it.

URL:
    The URL of the project repository to run the wizard on this server. If no URL is passed than will execute the wizard for each line on the qinpel-wiz.ini file.", env!("CARGO_PKG_VERSION"));
}
