use std::collections::HashMap;
use std::process::Command;
use std::str::from_utf8;

use anyhow::Error;
use serde_json::value::Value;

use crate::urlencode::urlencode;

pub type Metadata = HashMap<String, Entry>;

#[derive(Debug)]
pub struct Entry {
    name: String,
    source: Source,
    repo: Repo,
}

impl Entry {
    pub fn link(&self) -> Option<String> {
        match self.source {
            Source::CratesIo => Some(format!("https://crates.io/crates/{}", self.name)),
            Source::Unknown if !self.repo.is_empty() => Some(self.repo.url.clone()),
            _ => None,
        }
    }

    pub fn compare_url<A: AsRef<str>, B: AsRef<str>>(&self, from: A, to: B) -> Option<String> {
        use RepoKind::*;
        match self.repo.kind {
            Unknown => None,
            Github => Some(format!(
                "{}/compare/{}...{}",
                self.repo.url.trim_end_matches(".git"),
                urlencode(from.as_ref()),
                urlencode(to.as_ref())
            )),
        }
    }
}

#[derive(Debug)]
enum Source {
    Unknown,
    CratesIo,
}

impl From<String> for Source {
    fn from(s: String) -> Self {
        if s == "registry+https://github.com/rust-lang/crates.io-index" {
            Source::CratesIo
        } else {
            Source::Unknown
        }
    }
}

#[derive(Debug)]
enum RepoKind {
    Unknown,
    Github,
}

impl From<&str> for RepoKind {
    fn from(url: &str) -> Self {
        if url.starts_with("https://github.com/") {
            RepoKind::Github
        } else {
            RepoKind::Unknown
        }
    }
}

#[derive(Debug)]
struct Repo {
    kind: RepoKind,
    url: String,
}

impl From<String> for Repo {
    fn from(url: String) -> Self {
        Self {
            kind: url.as_str().into(),
            url,
        }
    }
}

impl Repo {
    fn is_empty(&self) -> bool {
        self.url.is_empty()
    }
}

fn format_err<T: std::fmt::Display>(s: T) -> anyhow::Error {
    Error::msg(format!("unexpected metadata format; {}", s))
}

trait StringFromExt {
    fn get_as_string(&self, key: &str) -> Result<Option<String>, Error>;
}

impl StringFromExt for serde_json::Map<String, Value> {
    fn get_as_string(&self, key: &str) -> Result<Option<String>, Error> {
        match self
            .get(key)
            .ok_or_else(|| format_err(format!("expected key '{}' is missing", key)))?
        {
            Value::String(s) => Ok(Some(s.into())),
            Value::Null => Ok(None),
            _ => Err(format_err(format!("'{}' is not a string", key))),
        }
    }
}

pub fn load_metadata() -> Result<Metadata, Error> {
    let mut metadata = HashMap::new();

    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--all-features")
        .arg("--offline")
        .output()?;

    if !output.status.success() {
        let stderr: String = from_utf8(&output.stderr).map(Into::into)?;
        return Err(Error::msg(stderr.trim_end().to_string()));
    }

    let json: serde_json::Map<String, Value> = serde_json::from_slice(&output.stdout)?;
    let packages = match json.get("packages") {
        Some(Value::Array(packages)) => packages,
        _ => return Err(format_err("'packages' key is missing")),
    };

    for pkg in packages {
        let map = match pkg {
            Value::Object(map) => map,
            _ => return Err(format_err("packages is not an array of objects")),
        };

        let name = map
            .get_as_string("name")?
            .ok_or_else(|| format_err("package name is empty"))?;
        let source = map.get_as_string("source")?.unwrap_or_default().into();
        let repo = map.get_as_string("repository")?.unwrap_or_default().into();

        metadata.insert(name.clone(), Entry { name, source, repo });
    }

    Ok(metadata)
}
