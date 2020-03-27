use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

use cargo_lock::Lockfile;

use crate::{verbose, Error, ErrorMsg};

pub fn load(source: &str, base: &str) -> Result<Lockfile, Error> {
    let base_path = if base.is_empty() {
        Path::new(".")
    } else {
        Path::new(base.trim_end_matches(std::path::MAIN_SEPARATOR))
    };

    let path = if source.is_empty() {
        base_path.join("Cargo.lock")
    } else {
        base_path.join(source)
    };

    if path.exists() {
        return Ok(Lockfile::load(path)?);
    }

    match load_git(source, base_path) {
        Ok(f) => Ok(f),
        Err(e) => {
            //
            // Try the next one. load_hg()?
            //
            Err(Error::with_err(ErrorMsg::NotFound, e))
        }
    }
}

fn load_git(gitref: &str, base_path: &Path) -> Result<Lockfile, Error> {
    let gitpath: OsString = if gitref.contains(':') {
        gitref.into()
    } else {
        let path = base_path.join("Cargo.lock");
        let mut gitpath = OsString::from(gitref);
        gitpath.push(":");
        gitpath.push(path);

        gitpath
    };

    let output = Command::new("git").arg("show").arg(&gitpath).output()?;

    if !output.status.success() {
        let code = output.status.code().unwrap_or(1);
        verbose(|| {
            let stderr: String = from_utf8(&output.stderr)
                .map(Into::into)
                .unwrap_or_else(|e| format!("{}", e));
            format!(
                "failed to load from git path {}; {}",
                gitpath.into_string().unwrap_or_else(|_| "bad path".into()),
                stderr.trim_end()
            )
        });
        let e = io::Error::from_raw_os_error(code);
        return Err(Error::with_err(ErrorMsg::CommandFailed("git"), e));
    }

    Ok(from_utf8(&output.stdout)?.parse()?)
}
