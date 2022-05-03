use anyhow::{Context, Result};
use std::collections::HashMap;

use git2::Repository;

pub trait RepoContainer {
    fn to_string(&self) -> String;
    fn find(&self, name: &str) -> Option<&Repository>;
}

impl RepoContainer for HashMap<String, Repository> {
    fn to_string(&self) -> String {
        let mut return_string = String::new();
        for name in self.keys() {
            return_string.push_str(&format!("{}\n", name));
        }
        return_string
    }

    fn find(&self, name: &str) -> Option<&Repository> {
        self.get(name)
    }
}

impl RepoContainer for Vec<Repository> {
    fn to_string(&self) -> String {
        let mut return_string = String::new();
        for repo in self {
            if repo.is_bare() {
                return_string.push_str(&format!(
                    "{}\n",
                    repo.path().file_name().unwrap().to_str().unwrap()
                ));
            } else {
                return_string.push_str(&format!(
                    "{}\n",
                    repo.path()
                        .parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                ));
            }
        }
        return_string
    }
    fn find(&self, name: &str) -> Option<&git2::Repository> {
        for repo in self {
            if repo.is_bare() {
                let temp = repo.path().file_name().unwrap().to_str().unwrap();
                if temp == name {
                    return Some(repo);
                }
            } else {
                let temp = repo
                    .path()
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();
                if temp == name {
                    return Some(repo);
                }
            }
        }
        None
    }
}

pub trait DirtyUtf8Path {
    fn to_string(&self) -> Result<String>;
}
impl DirtyUtf8Path for std::path::PathBuf {
    fn to_string(&self) -> Result<String> {
        Ok(self.to_str().context("Not a valid utf8 path")?.to_string())
    }
}
impl DirtyUtf8Path for std::path::Path {
    fn to_string(&self) -> Result<String> {
        Ok(self.to_str().context("Not a valid utf8 path")?.to_string())
    }
}
impl DirtyUtf8Path for std::ffi::OsStr {
    fn to_string(&self) -> Result<String> {
        Ok(self.to_str().context("Not a valid utf8 path")?.to_string())
    }
}
