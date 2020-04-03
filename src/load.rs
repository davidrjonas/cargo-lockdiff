use std::ffi::OsString;
use std::fmt;
use std::io;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;
use std::str::FromStr;

use anyhow::{Context, Error, Result};

use cargo_lock::Lockfile;

const ALL_SOURCES: [Source; 3] = [Source::File, Source::Git, Source::Mercurial];

#[derive(Clone, Debug)]
pub enum Source {
    File,
    Git,
    Mercurial,
}

impl Source {
    pub fn all() -> &'static [Source] {
        &ALL_SOURCES
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Git => "git",
            Self::Mercurial => "hg",
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Source {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "file" => Source::File,
            "git" => Source::Git,
            "hg" => Source::Mercurial,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
struct LoadError {
    errors: Vec<Error>,
}

impl std::error::Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for e in &self.errors {
            writeln!(f)?;
            write!(f, "  {:#}", e)?;
        }

        Ok(())
    }
}

pub fn load(sources: &[Source], fileish: &str, base: &str) -> Result<Lockfile> {
    let base_path = if base.is_empty() {
        Path::new(".")
    } else {
        Path::new(base.trim_end_matches(std::path::MAIN_SEPARATOR))
    };

    let mut err = LoadError { errors: vec![] };

    for source in sources {
        match source.load(fileish, base_path).context(source.as_str()) {
            Ok(f) => return Ok(f),
            Err(e) => err.errors.push(e),
        }
    }

    Err(err.into())
}

impl Source {
    fn load(&self, fileish: &str, base_path: &Path) -> Result<Lockfile> {
        match self {
            Self::File => load_file(fileish, base_path),
            Self::Git => load_git(fileish, base_path),
            Self::Mercurial => load_hg(fileish, base_path),
        }
    }
}

fn load_file(fileish: &str, base_path: &Path) -> Result<Lockfile> {
    let path = if fileish.is_empty() {
        base_path.join("Cargo.lock")
    } else {
        base_path.join(fileish)
    };

    if path.exists() {
        Ok(Lockfile::load(path)?)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, path.display().to_string()).into())
    }
}

fn load_git(gitref: &str, base_path: &Path) -> Result<Lockfile> {
    let gitpath: OsString = if gitref.contains(':') {
        gitref.into()
    } else {
        let mut gitpath = OsString::from(gitref);
        gitpath.push(":");
        gitpath.push(base_path.join("Cargo.lock"));

        gitpath
    };

    let output = Command::new("git").arg("show").arg(&gitpath).output()?;

    if !output.status.success() {
        let stderr: String = from_utf8(&output.stderr).map(Into::into)?;
        return Err(Error::msg(stderr.trim_end().to_string()));
    }

    Ok(from_utf8(&output.stdout)?.parse()?)
}

fn load_hg(fileish: &str, base_path: &Path) -> Result<Lockfile> {
    let mut file: OsString = base_path.join("Cargo.lock").into();

    // If they don't supply anything or they use the git default, convert to hg
    let hgref: OsString = if fileish.is_empty() || fileish == "HEAD" {
        ".".into()
    // If they use the colon separated git format take their word for the file path.
    } else if fileish.contains(':') {
        let mut split = fileish.splitn(2, ':');
        let hgref = split.next().expect("split 1");
        file = split.next().expect("split 2").into();
        hgref.into()
    // And sometimes a ref is just a ref
    } else {
        fileish.into()
    };

    let output = Command::new("hg")
        .arg("cat")
        .arg("-r")
        .arg(&hgref)
        .arg(&file)
        .output()?;

    if !output.status.success() {
        let stderr: String = from_utf8(&output.stderr).map(Into::into)?;
        return Err(Error::msg(stderr.trim_end().to_string()));
    }

    Ok(from_utf8(&output.stdout)?.parse()?)
}
