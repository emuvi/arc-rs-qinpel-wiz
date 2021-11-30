use octocrab::Octocrab;
use std::error::Error;
use std::marker::Send;
use std::marker::Sync;

use std::env;

mod locks;
mod repos;
mod tools;
mod wizlua;

pub type WizError = Box<dyn Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), WizError> {
    let mut magic = true;
    for arg in env::args() {
        if arg == "-v" || arg == "--version" {
            magic = false;
            let version = env!("CARGO_PKG_VERSION");
            println!("QinpelWiz {}", version);
        } else if arg.ends_with(".lua") {
            magic = false;
            match wizlua::execute(arg) {
                Ok(result) => println!("{}", result),
                Err(error) => eprintln!("{}", error),
            };
        } else if arg == "-m" || arg == "--magic" {
            magic = true;
        }
    }
    if magic {
        wizard().await?;
    }
    Ok(())
}

async fn wizard() -> Result<(), WizError> {
    println!("Starting Qinpel Wizard...");
    std::fs::create_dir_all("./code")?;
    std::fs::create_dir_all("./run")?;
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let github = Octocrab::builder().personal_token(token).build()?;
    println!("--- Getting all Qinpel repositories... ---");
    let repos = repos::get_qinpel_repos(github).await?;
    for repo in repos {
        println!("--- Starting Qinpel wizard of {} ---", repo.name);
        if let Err(e) = repo.wizard().await {
            eprintln!("Problem on wizard of {} - {}", repo.name, e);
        }
    }
    Ok(())
}
