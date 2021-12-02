use crate::locks::Locker;
use crate::WizError;
use liz;
use octocrab::Octocrab;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Repository {
	pub owner: String,
	pub name: String,
	pub path: PathBuf,
	pub lua_path: PathBuf,
}

impl Repository {
	pub async fn wizard(&self) -> Result<(), WizError> {
		if !self.path.exists() {
			let origin = format!("https://github.com/{}/{}", self.owner, self.name);
			println!("Cloning the repository from {}", origin);
			liz::tools::cmd("git", &["clone", &origin], "./code", true, true)?;
		} else {
			println!("Pulling the repository...");
			liz::tools::cmd("git", &["checkout", "master"], &self.path, true, true)?;
			liz::tools::cmd("git", &["reset", "--hard", "HEAD"], &self.path, true, true)?;
			liz::tools::cmd("git", &["clean", "-f", "-d", "-x"], &self.path, true, true)?;
			liz::tools::cmd("git", &["fetch", "--all", "--prune"], &self.path, true, true)?;
			liz::tools::cmd("git", &["pull"], &self.path, true, true)?;
		}
		println!("Starting to check on lua wizard...");
		let actual_tag = self.get_actual_tag()?;
		if actual_tag.is_empty() {
			self.lua_execute_with_no_tag()?;
		} else {
			self.lua_execute_with_actual_tag(actual_tag)?;
		}
		Ok(())
	}

	fn get_actual_tag(&self) -> Result<String, WizError> {
		let result = liz::tools::cmd(
			"git",
			&["tag", "--sort=-version:refname"],
			&self.path,
			false,
			true,
		)?;
		let actual_tag = result.1;
		let actual_tag = actual_tag.lines().next();
		if let Some(actual_tag) = actual_tag {
			Ok(String::from(actual_tag.trim()))
		} else {
			Ok(String::new())
		}
	}

	fn lua_execute_with_no_tag(&self) -> Result<(), WizError> {
		println!("The lua wizard will be executed while there is no tags.");
		self.lua_execute()?;
		Ok(())
	}

	fn lua_execute_with_actual_tag(&self, actual_tag: String) -> Result<(), WizError> {
		let mut locker = Locker::load()?;
		let mut should_run = true;
		if let Some(tag_done) = locker.locked.get(&self.name) {
			if &actual_tag == tag_done {
				println!(
					"The lua wizard was already executed for the actual tag: {}",
					actual_tag
				);
				should_run = false;
			}
		}
		if should_run {
			println!(
				"The lua wizard needs to be executed for the actual tag: {}",
				actual_tag
			);
			let tag_param = format!("tags/{}", actual_tag);
			liz::tools::cmd("git", &["checkout", &tag_param], &self.path, true, true)?;
			self.lua_execute()?;
			liz::tools::cmd("git", &["checkout", "master"], &self.path, true, true)?;
			locker
				.locked
				.insert(String::from(&self.name), String::from(actual_tag));
			locker.save()?;
		}
		Ok(())
	}

	fn lua_execute(&self) -> Result<(), WizError> {
		if !self.lua_path.exists() {
			println!("There is no lua wizard to be executed.");
		} else {
			println!("Starting to execute the lua wizard...");
			let result = liz::execute(&self.lua_path)?;
			println!("{}", result);
		}
		Ok(())
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
				let lua_path = format!("./code/{}/{}", name, "qinpel-wiz.lua");
				let lua_path = PathBuf::from(lua_path);
				result.push(Repository {
					owner,
					name,
					path,
					lua_path,
				});
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
