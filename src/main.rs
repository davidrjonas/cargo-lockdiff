//use std::path::Path;
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
        default = "Cargo.lock",
        help = "The  file, git ref, or git ref with filename to compare from."
    )]
    from: String,

    #[options(
        no_short,
        default = "HEAD:Cargo.lock",
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

    dbg!(&opts);
    let lockfile = Lockfile::load(opts.from).unwrap();

    let mut table = Table::new();

    table.set_format(format_markdown());
    table.set_titles(row!["Package", "Version"]);

    for pkg in &lockfile.packages {
        table.add_row(row![pkg.name, pkg.version]);
        if pkg.name.as_str() == "debug-helper" {
            println!("{:?}", pkg);
        }
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
