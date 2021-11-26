use octocrab::Octocrab;
use std::error::Error;

use std::env;

mod repos;
mod tools;
mod wizlua;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut magic = true;
    for arg in env::args() {
        if arg == "-v" || arg == "--version" {
            let version = env!("CARGO_PKG_VERSION");
            println!("QinpelWiz {}", version);
            magic = false;
        } else if arg.ends_with(".lua") {
            wizlua::execute(arg)?;
            magic = false;
        }
    }
    if magic {
        wizard().await?;
    }
    Ok(())
}

async fn wizard() -> Result<(), Box<dyn Error>> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let github = Octocrab::builder().personal_token(token).build()?;
    let repos = repos::get_qinpel_repos(github).await?;
    for repo in repos {
        repo.wizard().await?;
    }
    Ok(())
}
