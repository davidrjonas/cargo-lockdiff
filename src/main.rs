use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use gumdrop::Options;
use prettytable::{cell, format::TableFormat, row, Table};

mod diff;
mod load;

use diff::*;
use load::*;

static VERBOSE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Options)]
struct Opts {
    #[options(help = "Print help message")]
    help: bool,

    #[options(
        //default = "",
        help = "Base to with which to prefix paths. E.g. `-p app` would look for HEAD:app/Cargo.lock and app/Cargo.lock"
    )]
    path: String,

    #[options(
        no_short,
        default = "HEAD",
        help = "The  file, git ref, or git ref with filename to compare from."
    )]
    from: String,

    #[options(
        no_short,
        help = "The file, git ref, or git ref with filename to compare to."
    )]
    to: String,

    // #[options(no_short, help = "Only include changes from `dependencies`")]
    // only_prod: bool,

    // #[options(no_short, help = "Only include changes from `dev-dependencies`")]
    // only_dev: bool,
    #[options(short = "l", help = "Include links to where possible")]
    links: bool,

    #[options(short = "f", default = "markdown", help = "Output format: markdown")]
    format: Format,

    #[options(short = "v", help = "Show some extra messages")]
    verbose: bool,
}

#[derive(Debug)]
enum Format {
    Markdown,
}

#[derive(Debug, PartialEq)]
enum ErrorMsg {
    None,
    NotFound,
    CommandFailed(&'static str),
}

impl fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use ErrorMsg::*;
        match self {
            None => return Ok(()),
            NotFound => f.write_str("not found"),
            CommandFailed(cmd) => write!(f, "'{}' command failed", cmd),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    msg: ErrorMsg,
    source: Option<Box<dyn std::error::Error + 'static>>,
}

impl Error {
    fn with_err<E: std::error::Error + 'static>(msg: ErrorMsg, err: E) -> Self {
        Error {
            msg,
            source: Some(Box::new(err)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.msg != ErrorMsg::None {
            self.msg.fmt(f)?;
        }

        if let Some(err) = &self.source {
            if self.msg != ErrorMsg::None {
                f.write_str("\nErrors:\n  + ")?;
            }
            write!(f, "{:?}", err)?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|v| v.as_ref())
    }
}

impl From<ErrorMsg> for Error {
    fn from(msg: ErrorMsg) -> Self {
        Self { msg, source: None }
    }
}

impl From<cargo_lock::error::Error> for Error {
    fn from(e: cargo_lock::error::Error) -> Self {
        Self {
            msg: ErrorMsg::None,
            source: Some(Box::new(e)),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self {
            msg: ErrorMsg::None,
            source: Some(Box::new(e)),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Self {
            msg: ErrorMsg::None,
            source: Some(Box::new(e)),
        }
    }
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" => Ok(Format::Markdown),
            _ => Err("unknown format; try markdown"),
        }
    }
}

fn verbose<F: FnOnce() -> String>(msg_fn: F) {
    if VERBOSE.load(Relaxed) {
        eprintln!("{}", msg_fn());
    }
}

fn main() -> Result<(), i32> {
    let mut args = std::env::args().collect::<Vec<_>>();

    if let Some(v) = args.get(1) {
        if v == "lock-diff" {
            args.remove(1);
        }
    }

    let opts = Opts::parse_args_default(&args[1..]).map_err(|e| {
        eprintln!("{}: {}", args[0], e);
        2
    })?;

    if opts.help {
        println!("usage: {} [options]", args[0]);
        println!("");
        println!("{}", Opts::usage());
        return Ok(());
    }

    if opts.verbose {
        VERBOSE.store(true, Relaxed);
    }

    let from = load(&opts.from, &opts.path).map_err(|e| {
        eprintln!("could not read 'from'; {}", e);
        1
    })?;

    let to = load(&opts.to, &opts.path).map_err(|e| {
        eprintln!("could not read 'to'; {}", e);
        1
    })?;

    let diff = diff(&from, &to);

    if diff.len() > 0 {
        print_difftable(&diff, format_markdown(), opts.links);
    } else {
        println!("No changes");
    }

    Ok(())
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

fn print_difftable(diff: &Diff, format: TableFormat, links: bool) {
    let mut table = Table::new();

    table.set_format(format);
    table.set_titles(row!["Package", "From", "To"]);

    for (name, changes) in diff {
        let col0 = match &changes.link {
            Some(link) if links => format!("[{}][{}]", name, link.id),
            _ => name.clone(),
        };

        table.add_row(row![col0, changes.from, changes.to]);
    }

    table.printstd();

    if links {
        println!("");

        for (_, changes) in diff {
            if let Some(link) = &changes.link {
                println!("[{}]: {}", link.id, link.url);
            }
        }
    }
}
