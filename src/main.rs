use clap::Parser;
use eyre::{eyre, Result};
use prettytable::{cell, row, Table};

mod diff;
mod load;
mod metadata;
mod urlencode;

use diff::*;
use load::*;
use metadata::*;

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
struct Opts {
    /// Base to with which to prefix paths. E.g. `-p app` would look for HEAD:app/Cargo.lock and app/Cargo.lock
    #[clap(long, default_value = "", env = "CARGO_LOCKDIFF_PATH")]
    path: String,

    /// The file, vcs ref, or vcs ref with filename to compare from. To force the use of a
    /// particular vcs, prepend it with a colon. E.g. "hg:.". The ref is required if the vcs is
    /// specified.
    #[clap(long, default_value = "HEAD", env = "CARGO_LOCKDIFF_FROM")]
    from: String,

    /// The file, vcs ref, or vcs ref with filename to compare to. To force the use of a
    /// particular vcs, prepend it with a colon. E.g. "hg:.". The ref is required if the vcs is
    /// specified.
    #[clap(long, default_value = "", env = "CARGO_LOCKDIFF_TO")]
    to: String,

    /// Do not include links (links requires `cargo metadata` which may require network ops)
    #[clap(short = 'n', long, env = "CARGO_LOCKDIFF_NO_LINKS")]
    no_links: bool,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let (from_sources, from_fileish) = parse_source_opt(&opts.from);

    let from = load(&from_sources, from_fileish, &opts.path)
        .map_err(|e| eyre!("could not read 'from': {}", e))?;

    let (to_sources, to_fileish) = parse_source_opt(&opts.to);

    let to = load(&to_sources, to_fileish, &opts.path)
        .map_err(|e| eyre!("could not read 'to': {}", e))?;

    let diff = diff(&from, &to);

    if !diff.is_empty() {
        if opts.no_links {
            print_markdown_no_links(&diff);
        } else {
            print_markdown(&diff, load_metadata()?);
        }
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

fn print_markdown_no_links(diff: &Diff) {
    use prettytable::format::{FormatBuilder, LinePosition::*, LineSeparator};

    let format = FormatBuilder::new()
        .borders('|')
        .column_separator('|')
        .separator(Title, LineSeparator::new('-', '|', '|', '|'))
        .padding(1, 1)
        .build();

    let mut table = Table::new();

    table.set_format(format);
    table.set_titles(row!["Package", "From", "To", "Compare"]);

    for (name, changes) in diff {
        table.add_row(row![name, changes.from, changes.to]);
    }

    table.printstd();
}

fn print_markdown(diff: &Diff, metadata: Metadata) {
    use prettytable::format::{FormatBuilder, LinePosition::*, LineSeparator};

    let format = FormatBuilder::new()
        .borders('|')
        .column_separator('|')
        .separator(Title, LineSeparator::new('-', '|', '|', '|'))
        .padding(1, 1)
        .build();

    let mut table = Table::new();

    table.set_format(format);
    table.set_titles(row!["Package", "From", "To", "Compare"]);

    let mut linked: Vec<(u32, String)> = Vec::new();
    let mut count = 0;

    for (name, changes) in diff {
        let col0 = match metadata.get(name).map(|v| v.link()).flatten() {
            Some(link) => {
                count += 1;
                linked.push((count, link));
                format!("[{}][{:02X}]", name, count)
            }
            None => name.clone(),
        };

        let should_compare = changes.from != Version::New && changes.to != Version::Removed;

        let compare = if should_compare {
            match metadata
                .get(name)
                .map(|entry| entry.compare_url(changes.from.to_string(), changes.to.to_string()))
                .flatten()
            {
                Some(url) => {
                    count += 1;
                    linked.push((count, url));
                    format!("[...][{:02X}]", count)
                }
                None => "".into(),
            }
        } else {
            "".into()
        };

        table.add_row(row![col0, changes.from, changes.to, compare]);
    }

    table.printstd();

    println!();

    for (id, url) in linked {
        println!("[{:02X}]: {}", id, url);
    }
}
