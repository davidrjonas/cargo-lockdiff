use anyhow::{anyhow, Result};
use prettytable::{cell, row, Table};

mod diff;
mod load;

use diff::*;
use load::*;

#[derive(Debug)]
struct Opts {
    /// base to with which to prefix paths. E.g. `-p app` would look for HEAD:app/Cargo.lock and app/Cargo.lock
    path: String,

    /// the file, vcs ref, or vcs ref with filename to compare from. To force the use of a
    /// particular vcs, prepend it with a colon. E.g. "hg:.". The ref is required if the vcs is
    /// specified.
    from: String,

    /// the file, vcs ref, or vcs ref with filename to compare to. To force the use of a
    /// particular vcs, prepend it with a colon. E.g. "hg:.". The ref is required if the vcs is
    /// specified.
    to: String,

    /// do not include links
    no_links: bool,
}

#[paw::main]
fn main(opts: Opts) -> Result<()> {
    let (from_sources, from_fileish) = parse_source_opt(&opts.from);

    let from = load(&from_sources, from_fileish, &opts.path)
        .map_err(|e| anyhow!("could not read 'from': {}", e))?;

    let (to_sources, to_fileish) = parse_source_opt(&opts.to);

    let to = load(&to_sources, to_fileish, &opts.path)
        .map_err(|e| anyhow!("could not read 'to': {}", e))?;

    let diff = diff(&from, &to);

    if diff.len() > 0 {
        print_markdown(&diff, !opts.no_links);
    } else {
        println!("No changes");
    }

    Ok(())
}

fn parse_source_opt(opt: &str) -> (Vec<Source>, &str) {
    let mut iter = opt.splitn(2, ':');
    let head = iter.next();
    let rest = iter.next();

    if opt.is_empty() || rest.is_none() {
        return (Source::all().to_vec(), opt);
    }

    let source = match head.expect("head").parse::<Source>() {
        Ok(s) => s,
        Err(_) => {
            return (Source::all().to_vec(), opt);
        }
    };

    (vec![source], rest.unwrap_or_default())
}

fn print_markdown(diff: &Diff, links: bool) {
    use prettytable::format::{FormatBuilder, LinePosition::*, LineSeparator};

    let format = FormatBuilder::new()
        .borders('|')
        .column_separator('|')
        .separator(Title, LineSeparator::new('-', '|', '|', '|'))
        .padding(1, 1)
        .build();

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

fn print_help() {
    println!(
        r#"Usage: cargo lockdiff [-p <path>] [--from <from>] [--to <to>] [-l]

Diff your lock file, see what's changed. Use these options or environment variables prefixed with
`CARGO_LOCKDIFF_`, such as `CARGO_LOCKDIFF_NO_LINKS=true`.

Options:
  -p, --path        Base to with which to prefix paths. E.g. `-p app` would look
                    for HEAD:app/Cargo.lock and app/Cargo.lock. Env: CARGO_LOCKDIFF_PATH

  --from            The file, vcs ref, or vcs ref with filename to compare from.
                    To force the use of a particular vcs, prepend it with a
                    colon. E.g. "hg:.". The ref is required if the vcs is
                    specified. Env: CARGO_LOCKDIFF_FROM

  --to              The file, vcs ref, or vcs ref with filename to compare to.
                    To force the use of a particular vcs, prepend it with a
                    colon. E.g. "hg:.". The ref is required if the vcs is
                    specified. Env: CARGO_LOCKDIFF_TO

  -n, --no-links    Include links to where possible. Env: CARGO_LOCKDIFF_NO_LINKS

  -h, --help        Display usage information
    "#
    );

    std::process::exit(0);
}

impl paw::ParseArgs for Opts {
    type Error = anyhow::Error;

    fn parse_args() -> Result<Self, Self::Error> {
        use std::env;

        let mut args = env::args().skip(1);

        let mut opts = Opts {
            path: env::var("CARGO_LOCKDIFF_PATH").unwrap_or_default(),
            from: env::var("CARGO_LOCKDIFF_FROM").unwrap_or("HEAD".into()),
            to: env::var("CARGO_LOCKDIFF_TO").unwrap_or_default(),
            no_links: env::var("CARGO_LOCKDIFF_NO_LINKS")
                .map(|v| v.parse().unwrap_or(false))
                .unwrap_or(false),
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "lockdiff" => {}
                "-p" | "--path" => {
                    opts.path = args.next().ok_or_else(|| anyhow!("expected path"))?
                }
                "--to" => {
                    opts.to = args
                        .next()
                        .ok_or_else(|| anyhow!("expected 'to' fileish"))?
                }
                "--from" => {
                    opts.from = args
                        .next()
                        .ok_or_else(|| anyhow!("expected 'from' fileish"))?
                }
                "-n" | "--no-links" => opts.no_links = true,
                "-h" | "--help" => print_help(),
                arg => return Err(anyhow!("Unknown argument '{}'", arg)),
            }
        }

        Ok(opts)
    }
}
