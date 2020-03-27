use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;
use std::str::FromStr;

use cargo_lock::Lockfile;
use gumdrop::Options;
use prettytable::{cell, format::TableFormat, row, Table};

#[derive(Debug, Options)]
struct Opts {
    #[options(help = "Print help message")]
    help: bool,

    #[options(
        default = "./",
        help = "Base to with which to prefix paths. E.g. `-p app` would look for HEAD:app/Cargo.lock and app/Cargo.lock"
    )]
    path: String,

    #[options(
        no_short,
        default = "HEAD:Cargo.lock",
        help = "The  file, git ref, or git ref with filename to compare from."
    )]
    from: String,

    #[options(
        no_short,
        default = "Cargo.lock",
        help = "The file, git ref, or git ref with filename to compare to."
    )]
    to: String,

    #[options(no_short, help = "Only include changes from `dependencies`")]
    only_prod: bool,

    #[options(no_short, help = "Only include changes from `dev-dependencies`")]
    only_dev: bool,

    #[options(no_short, help = "Do not include any links")]
    no_links: bool,

    #[options(no_short, default = "markdown", help = "Do not include any links")]
    format: Format,
}

#[derive(Debug)]
enum Format {
    Markdown,
    Json,
    PrettyJson,
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" => Ok(Format::Markdown),
            "json" => Ok(Format::Json),
            "prettyjson" => Ok(Format::PrettyJson),
            _ => Err("unknown format; try markdown, json, or prettyjson"),
        }
    }
}

fn main() {
    let opts = Opts::parse_args_default_or_exit();

    let from = read_lockfile(opts.from).unwrap();
    let to = read_lockfile(opts.to).unwrap();

    let diff = diff(&from, &to);

    let mut table = Table::new();

    table.set_format(format_markdown());
    table.set_titles(row!["Package", "From", "To"]);

    for (name, changes) in diff {
        table.add_row(row![name, changes.from, changes.to]);
    }

    table.printstd();
}

fn format_markdown() -> TableFormat {
    use prettytable::format::{FormatBuilder, LinePosition::*, LineSeparator};

    FormatBuilder::new()
        .borders('|')
        .column_separator('|')
        .separator(Title, LineSeparator::new('-', '|', '|', '|'))
        .padding(1, 1)
        .build()
}

fn read_lockfile(source: String) -> Result<Lockfile, io::Error> {
    let path = Path::new(&source);

    if path.exists() {
        Ok(Lockfile::load(path).unwrap())
    } else {
        let output = Command::new("env")
            .arg("git")
            .arg("show")
            .arg(source)
            .output()?;

        if !output.status.success() {
            let e = io::Error::from_raw_os_error(output.status.code().unwrap_or(1));
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }

        from_utf8(&output.stdout)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

struct Changes {
    name: String,
    from: Version,
    to: Version,
}

type Diff = HashMap<String, Changes>;

#[derive(Debug, PartialEq)]
enum Version {
    New,
    Removed,
    At(cargo_lock::Version),
}

fn diff(from: &Lockfile, to: &Lockfile) -> Diff {
    let mut diff = HashMap::new();

    for pkg in &from.packages {
        let changes = Changes {
            name: pkg.name.as_str().to_owned(),
            from: Version::At(pkg.version.clone()),
            to: Version::Removed,
        };

        diff.insert(changes.name.clone(), changes);
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
            let c = Changes {
                name: pkg.name.as_str().to_owned(),
                from: Version::New,
                to: Version::At(pkg.version.clone()),
            };

            diff.insert(c.name.clone(), c);
        }
    }

    diff
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
