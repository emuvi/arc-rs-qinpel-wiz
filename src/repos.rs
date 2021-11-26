use octocrab::Octocrab;
use serde_json::Value;
use std::error::Error;

use crate::tools;

#[derive(Debug)]
pub struct Repository {
    pub owner: String,
    pub name: String,
}

impl Repository {

    pub async fn wizard(&self) -> Result<(), Box<dyn Error>> {
        println!("{}", tools::command(
            "git",
            &["--version"],
            ".",
        )?);
        Ok(())
    }

}

pub async fn get_qinpel_repos(github: Octocrab) -> Result<Vec<Repository>, Box<dyn Error>> {
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
                result.push(Repository { owner, name });
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
