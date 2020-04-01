use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use argh::FromArgs;
use prettytable::{cell, format::TableFormat, row, Table};

mod diff;
mod load;

use diff::*;
use load::*;

static VERBOSE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, FromArgs)]
/// Diff your lock file, see what's changed.
struct Opts {
    /// base to with which to prefix paths. E.g. `-p app` would look for HEAD:app/Cargo.lock and app/Cargo.lock
    #[argh(option, short = 'p', default = r#""".into()"#)]
    path: String,

    /// the file, git ref, or git ref with filename to compare from
    #[argh(option, default = r#""HEAD".into()"#)]
    from: String,

    /// the file, git ref, or git ref with filename to compare to
    #[argh(option, default = r#""".into()"#)]
    to: String,

    /// include links to where possible
    #[argh(switch, short = 'l')]
    links: bool,

    /// show some extra messages
    #[argh(switch, short = 'v')]
    verbose: bool,
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

fn verbose<F: FnOnce() -> String>(msg_fn: F) {
    if VERBOSE.load(Relaxed) {
        eprintln!("{}", msg_fn());
    }
}

fn main() -> Result<(), i32> {
    let opts: Opts = match std::env::args().nth(1) {
        Some(arg) if arg == "lockdiff" => argh::cargo_from_env(),
        _ => argh::from_env(),
    };

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
        print_markdown(&diff, opts.links);
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

fn print_markdown(diff: &Diff, links: bool) {
    let format = format_markdown();

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
