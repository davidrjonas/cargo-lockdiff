use std::collections::BTreeMap;
use std::fmt;

use cargo_lock::{Lockfile, Package};

pub type Diff = BTreeMap<String, Changes>;

pub struct Changes {
    pub name: String,
    pub from: Version,
    pub to: Version,
    pub link: Option<Link>,
}

pub struct Link {
    pub id: String,
    pub url: String,
}

#[derive(Debug, PartialEq)]
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
    let mut counter: u32 = 0;

    for pkg in &from.packages {
        Changes::from_old(pkg, counter).insert(&mut diff);
        counter += 1;
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
            Changes::from_new(pkg, counter).insert(&mut diff);
            counter += 1;
        }
    }

    diff
}

impl Changes {
    fn insert(self, diff: &mut Diff) {
        diff.insert(self.name.clone(), self);
    }

    fn from_old(pkg: &Package, index: u32) -> Self {
        Self {
            name: pkg.name.as_str().to_owned(),
            from: Version::At(pkg.version.clone()),
            to: Version::Removed,
            link: Link::from_package(pkg, index),
        }
    }

    fn from_new(pkg: &Package, index: u32) -> Self {
        Self {
            name: pkg.name.as_str().to_owned(),
            from: Version::New,
            to: Version::At(pkg.version.clone()),
            link: Link::from_package(pkg, index),
        }
    }
}

impl Link {
    fn from_package(pkg: &Package, index: u32) -> Option<Self> {
        pkg.source
            .as_ref()
            .map(|s| {
                if s.is_default_registry() {
                    Some(Self {
                        id: format!("{}", index),
                        url: format!("https://crates.io/crates/{}", pkg.name.as_str()),
                    })
                } else {
                    None
                }
            })
            .flatten()
    }
}
