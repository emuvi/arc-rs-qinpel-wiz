use octocrab::Error;
use octocrab::Octocrab;
use serde_json::Value;

#[derive(Debug)]
struct Repository {
    owner: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let github = Octocrab::builder().personal_token(token).build()?;
    let all_repos = get_all_repos(&github).await?;
    let mut qinpel_base: Vec<Repository> = Vec::new();
    let mut qinpel_apps: Vec<Repository> = Vec::new();
    let mut qinpel_cmds: Vec<Repository> = Vec::new();
    for repo in all_repos {
        if repo.name.starts_with("qinpel-") {
            qinpel_base.push(repo);
        } else if repo.name.ends_with("-qap") {
            qinpel_apps.push(repo);
        } else if repo.name.ends_with("-qic") {
            qinpel_cmds.push(repo);
        }
    }
    wiz_qinpel_base(qinpel_base).await?;
    wiz_qinpel_apps(qinpel_apps).await?;
    wiz_qinpel_cmds(qinpel_cmds).await?;
    Ok(())
}

async fn wiz_qinpel_base(repos: Vec<Repository>) -> Result<(), Error> {
    for repo in repos {
        println!("Wizard Base {:?}", repo);
    }
    Ok(())
}

async fn wiz_qinpel_apps(repos: Vec<Repository>) -> Result<(), Error> {
    for repo in repos {
        println!("Wizard Application {:?}", repo);
    }
    Ok(())
}

async fn wiz_qinpel_cmds(repos: Vec<Repository>) -> Result<(), Error> {
    for repo in repos {
        println!("Wizard Command {:?}", repo);
    }
    Ok(())
}

async fn get_all_repos(github: &Octocrab) -> Result<Vec<Repository>, Error> {
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
            result.push(Repository { owner, name });
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
