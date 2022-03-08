use std::collections::BTreeMap;
use std::fmt;

use cargo_lock::{Lockfile, Package};

pub type Diff = BTreeMap<String, Changes>;

#[derive(Debug, serde::Serialize)]
pub struct Changes {
    pub name: String,
    pub from: Version,
    pub to: Version,
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub enum Version {
    New,
    Removed,
    At(cargo_lock::Version),
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Version::*;

        match self {
            New => f.write_str("NEW"),
            Removed => f.write_str("REMOVED"),
            At(v) => v.fmt(f),
        }
    }
}

pub fn diff(from: &Lockfile, to: &Lockfile) -> Diff {
    let mut diff = BTreeMap::new();

    for pkg in &from.packages {
        insert_changes(&mut diff, Changes::from_old(pkg))
    }

    for pkg in &to.packages {
        if let Some(changes) = diff.get_mut(pkg.name.as_str()) {
            let to = Version::At(pkg.version.clone());

            if changes.from != to {
                changes.to = to;
            } else {
                diff.remove(pkg.name.as_str());
            }
        } else {
            insert_changes(&mut diff, Changes::from_new(pkg))
        }
    }

    diff
}

fn insert_changes(diff: &mut Diff, ch: Changes) {
    diff.insert(ch.name.clone(), ch);
}

impl Changes {
    fn from_old(pkg: &Package) -> Self {
        Self {
            name: pkg.name.as_str().to_owned(),
            from: Version::At(pkg.version.clone()),
            to: Version::Removed,
        }
    }

    fn from_new(pkg: &Package) -> Self {
        Self {
            name: pkg.name.as_str().to_owned(),
            from: Version::New,
            to: Version::At(pkg.version.clone()),
        }
    }
}
