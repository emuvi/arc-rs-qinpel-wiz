use crate::locks::Locker;
use crate::WizError;
use liz;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Repository {
	pub address: String,
	pub name: String,
	pub code_path: PathBuf,
	pub wiz_path: PathBuf,
	pub cargo_path: PathBuf,
}

impl Repository {
	pub fn wizard(&self, clean: bool) -> Result<(), WizError> {
		if clean {
			liz::files::rm(&self.code_path)?;
		}
		if !self.code_path.exists() {
			println!("Cloning the repository from {}", self.address);
			liz::execs::cmd("git", &["clone", &self.address], "./code", true, true)?;
		} else {
			println!("Pulling the repository...");
			liz::execs::cmd("git", &["checkout", "master"], &self.code_path, true, true)?;
			liz::execs::cmd("git", &["pull"], &self.code_path, true, true)?;
		}
		println!("Starting to check on Qinpel wizard...");
		let actual_tag = self.get_actual_tag()?;
		if actual_tag.is_empty() {
			self.wiz_execute_with_no_tag()?;
		} else {
			self.wiz_execute_with_actual_tag(actual_tag)?;
		}
		Ok(())
	}

	fn get_actual_tag(&self) -> Result<String, WizError> {
		let result = liz::execs::cmd(
			"git",
			&["tag", "--sort=-version:refname"],
			&self.code_path,
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

	fn wiz_execute_with_no_tag(&self) -> Result<(), WizError> {
		println!("The Qinpel wizard will be executed while there is no tags.");
		self.wiz_execute()?;
		Ok(())
	}

	fn wiz_execute_with_actual_tag(&self, actual_tag: String) -> Result<(), WizError> {
		let mut locker = Locker::load()?;
		let mut should_run = true;
		if let Some(tag_done) = locker.locked.get(&self.name) {
			if &actual_tag == tag_done {
				println!(
					"The Qinpel wizard was already executed for the actual tag: {}",
					actual_tag
				);
				should_run = false;
			}
		}
		if should_run {
			println!(
				"The Qinpel wizard needs to be executed for the actual tag: {}",
				actual_tag
			);
			let tag_param = format!("tags/{}", actual_tag);
			liz::execs::cmd(
				"git",
				&["checkout", &tag_param],
				&self.code_path,
				true,
				true,
			)?;
			self.wiz_execute()?;
			liz::execs::cmd("git", &["checkout", "master"], &self.code_path, true, true)?;
			locker
				.locked
				.insert(String::from(&self.name), String::from(actual_tag));
			locker.save()?;
		}
		Ok(())
	}

	fn wiz_execute(&self) -> Result<(), WizError> {
		if !self.wiz_path.exists() {
			println!("There is no Qinpel wizard to be executed.");
			if self.cargo_path.exists() {
				println!("But it's a Cargo project so it will be deployed as such.");
				self.deploy_cargo()?;
			}
		} else {
			println!("Starting to execute the Qinpel wizard...");
			let results = liz::exec(&self.wiz_path, None)?;
			if results.is_empty() {
				println!("Qinpel wizard executed with no results.");
			} else {
				println!("Qinpel wizard executed with the results:");
				for result in results {
					println!("{}", result);
				}
			}
		}
		Ok(())
	}

	fn deploy_cargo(&self) -> Result<(), WizError> {
		liz::execs::cmd(
			"cargo",
			&["build", "--release"],
			&self.code_path,
			true,
			true,
		)?;
		let origin = format!(
			"./code/{}/target/release/{}{}",
			self.name,
			self.name,
			liz::files::exe_ext()
		);
		let destiny = format!(
			"./run/cmds/{}/{}{}",
			self.name,
			self.name,
			liz::files::exe_ext()
		);
		liz::files::cp_tmp(origin, destiny)?;
		Ok(())
	}
}

pub fn get_qinpel_repos() -> Result<Vec<Repository>, WizError> {
	let mut result: Vec<Repository> = Vec::new();
	let data = std::fs::read_to_string("./qinpel-wiz.ini")?;
	for line in data.lines() {
		if let Some(separator) = line.rfind('/') {
			let address = String::from(line);
			let name = String::from(&line[separator + 1..]);
			let code_path = format!("./code/{}", name);
			let code_path = PathBuf::from(code_path);
			let wiz_path = format!("./code/{}/{}", name, "qinpel-wiz.liz");
			let wiz_path = PathBuf::from(wiz_path);
			let cargo_path = format!("./code/{}/{}", name, "Cargo.toml");
			let cargo_path = PathBuf::from(cargo_path);
			result.push(Repository {
				address,
				name,
				code_path,
				wiz_path,
				cargo_path,
			});
		}
	}
	Ok(result)
}
