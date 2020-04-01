use anyhow::{anyhow, Result};
use argh::FromArgs;
use prettytable::{cell, row, Table};

mod diff;
mod load;

use diff::*;
use load::*;

#[derive(Debug, FromArgs)]
/// Diff your lock file, see what's changed.
struct Opts {
    /// base to with which to prefix paths. E.g. `-p app` would look for HEAD:app/Cargo.lock and app/Cargo.lock
    #[argh(option, short = 'p', default = r#""".into()"#)]
    path: String,

    /// the file, vcs ref, or vcs ref with filename to compare from. To force the use of a
    /// particular vcs, prepend it with a colon. E.g. "hg:.". The ref is required if the vcs is
    /// specified.
    #[argh(option, default = r#""HEAD".into()"#)]
    from: String,

    /// the file, vcs ref, or vcs ref with filename to compare to. To force the use of a
    /// particular vcs, prepend it with a colon. E.g. "hg:.". The ref is required if the vcs is
    /// specified.
    #[argh(option, default = r#""".into()"#)]
    to: String,

    /// include links to where possible
    #[argh(switch, short = 'l')]
    links: bool,
}

fn main() -> Result<()> {
    let opts: Opts = match std::env::args().nth(1) {
        Some(arg) if arg == "lockdiff" => argh::cargo_from_env(),
        _ => argh::from_env(),
    };

    let (from_sources, from_fileish) = parse_source_opt(&opts.from);

    let from = load(&from_sources, from_fileish, &opts.path)
        .map_err(|e| anyhow!("could not read 'from': {}", e))?;

    let (to_sources, to_fileish) = parse_source_opt(&opts.to);

    let to = load(&to_sources, to_fileish, &opts.path)
        .map_err(|e| anyhow!("could not read 'to': {}", e))?;

    let diff = diff(&from, &to);

    if diff.len() > 0 {
        print_markdown(&diff, opts.links);
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
