use octocrab;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), octocrab::Error> {
    let github = octocrab::instance();
    let response: serde_json::Value = github.graphql("query { viewer { login }}").await?;
    println!("{}", response);
    Ok(())
}
