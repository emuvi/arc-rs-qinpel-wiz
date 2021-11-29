use octocrab::Octocrab;
use serde_json::Value;
use std::path::PathBuf;
use crate::locks::Locker;
use crate::tools;
use crate::wizlua;
use crate::WizError;

#[derive(Debug)]
pub struct Repository {
    pub owner: String,
    pub name: String,
    pub path: PathBuf,
}

impl Repository {

    pub async fn wizard(&self) -> Result<(), WizError> {
        if self.path.exists() {
            println!("Pulling the repository...");
            tools::cmd("git", &["pull"], self.path, true)?;
        } else {
            let origin = format!("https://github.com/{}/{}", self.owner, self.name);
            println!("Cloning the repository from {}", origin);
            tools::cmd("git", &["clone", &origin], "./code", true)?;
        }
        println!("Starting to check on lua wizard...");
        let lua_path = format!("./code/{}/{}", self.name, "qinpel-wiz.lua");
        let lua_path = std::path::Path::new(&lua_path);
        if !lua_path.exists() {
            println!("There is no lua wizard on this repository.");
        } else {
            println!("Checking the necessity of execution of the lua wizard...");
            let mut locker = Locker::load()?;
            let actual_tag = tools::cmd("git", &["tag", "--sort=-version:refname"], path, false)?;
            let actual_tag = actual_tag.lines().next();
            if actual_tag.is_none() {
                println!("Could not execute the lua wizard because there is no tags in this repository.");
                return Ok(());
            }
            let actual_tag = actual_tag.unwrap();
            if actual_tag.is_empty() {
                println!("Could not execute the lua wizard because there is no tags in this repository.");
                return Ok(());
            }
            let mut should_run = true;

            if let Some(tag_done) =  locker.locked.get(&self.name) {
                if actual_tag == tag_done {
                    println!("The lua wizard was already executed for the actual tag: {}", actual_tag);
                    should_run = false;
                }
            }
            if should_run {
                println!("The lua wizard needs to be executed for the actual tag: {}", actual_tag);
                println!("Starting to execute the lua wizard...");
                let result = wizlua::execute(lua_path)?;
                println!("{}", result);
                locker.locked.insert(String::from(&self.name), String::from(actual_tag));
                locker.save()?;
            }
        }
        Ok(())
    }

    fn get_actual_tag(&self) -> String {

    }

}

pub async fn get_qinpel_repos(github: Octocrab) -> Result<Vec<Repository>, WizError> {
    let mut after: String = String::new();
    let mut result: Vec<Repository> = Vec::new();
    loop {
        let query = format!(
            "query {{
                viewer {{
                    repositories(first: 30 {}) {{
                        nodes {{
                            nameWithOwner
                        }}
                        pageInfo {{
                            hasNextPage
                            endCursor
                        }}
                    }}
                }}
            }}",
            after
        );
        let response: Value = github.graphql(&query).await?;
        let response = response.as_object().unwrap();
        let data = response.get("data").unwrap().as_object().unwrap();
        let viewer = data.get("viewer").unwrap().as_object().unwrap();
        let repositories = viewer.get("repositories").unwrap().as_object().unwrap();
        let nodes = repositories.get("nodes").unwrap().as_array().unwrap();
        for node in nodes {
            let node = node.as_object().unwrap();
            let name_with_owner = node.get("nameWithOwner").unwrap().as_str().unwrap();
            let mut name_parts = name_with_owner.split("/");
            let owner = String::from(name_parts.next().unwrap());
            let name = String::from(name_parts.next().unwrap());
            if is_qinpel_repo(&name) {
                let path = format!("./code/{}", name);
                let path = PathBuf::from(path);
                result.push(Repository { owner, name, path });
            }
        }
        let page_info = repositories.get("pageInfo").unwrap().as_object().unwrap();
        let has_next_page = page_info.get("hasNextPage").unwrap().as_bool().unwrap();
        if has_next_page {
            let end_cursor = page_info.get("endCursor").unwrap().as_str().unwrap();
            after = format!(", after: \"{}\"", end_cursor);
        } else {
            break;
        }
    }
    Ok(result)
}

fn is_qinpel_repo(name: &str) -> bool {
    return name.starts_with("qinpel-") || name.ends_with("-qap") || name.ends_with("-qic");
}
