cargo-lockdiff
==============

See what crates have changed after you run `cargo update` by comparing Cargo.lock to the the vcs HEAD or file of your choice.

Dependencies
------------

To use `git` or `hg` the respective binary must be found in `PATH`.

Install
=======

```bash
cargo install cargo-lockdiff

# try it
cargo lockdiff --help
```

Usage
=====

```bash
cargo update
# don't commit yet!
cargo lockdiff
```

Or from vim, to insert the output into the commit message, type `:r!cargo lockdiff`.

To see what changed long ago (in git),

```bash
cargo lockdiff --from "HEAD@{2 months ago}"
```

Options
-------

- `-p, --path <path>`: Base to with which to prefix paths. E.g. `-p app` would look for HEAD:app/Cargo.lock and app/Cargo.lock
- `--from <fileish>`: The file, vcs ref, or vcs ref with filename to compare from.  To force the use of a particular vcs, prepend it with a colon. E.g. "hg:."
- `--to <fileish>`: The file, vcs ref, or vcs ref with filename to compare to.  To force the use of a particular vcs, prepend it with a colon. E.g. "hg:."
- `-l, --links`: Include links to where possible
- `--help`: Display usage information

Example Output
==============

Raw
---

```
$ cargo lockdiff --links

| Package                    | From  | To      |
|----------------------------|-------|---------|
| cargo-lock-diff            | 0.1.0 | REMOVED |
| [anyhow][53]               | NEW   | 1.0.28  |
| [gumdrop_derive][19]       | 0.7.0 | REMOVED |
| [unicode-segmentation][59] | NEW   | 1.6.0   |
| [argh][54]                 | NEW   | 0.1.3   |
| [argh_derive][55]          | NEW   | 0.1.1   |
| [argh_shared][56]          | NEW   | 0.1.1   |
| [heck][58]                 | NEW   | 0.3.1   |
| [gumdrop][18]              | 0.7.0 | REMOVED |
| cargo-lockdiff             | NEW   | 0.1.0   |

[53]: https://crates.io/crates/anyhow
[19]: https://crates.io/crates/gumdrop_derive
[59]: https://crates.io/crates/unicode-segmentation
[54]: https://crates.io/crates/argh
[55]: https://crates.io/crates/argh_derive
[56]: https://crates.io/crates/argh_shared
[58]: https://crates.io/crates/heck
[18]: https://crates.io/crates/gumdrop
```

Rendered
--------

| Package                    | From  | To      |
|----------------------------|-------|---------|
| cargo-lock-diff            | 0.1.0 | REMOVED |
| [anyhow][53]               | NEW   | 1.0.28  |
| [gumdrop_derive][19]       | 0.7.0 | REMOVED |
| [unicode-segmentation][59] | NEW   | 1.6.0   |
| [argh][54]                 | NEW   | 0.1.3   |
| [argh_derive][55]          | NEW   | 0.1.1   |
| [argh_shared][56]          | NEW   | 0.1.1   |
| [heck][58]                 | NEW   | 0.3.1   |
| [gumdrop][18]              | 0.7.0 | REMOVED |
| cargo-lockdiff             | NEW   | 0.1.0   |

[53]: https://crates.io/crates/anyhow
[19]: https://crates.io/crates/gumdrop_derive
[59]: https://crates.io/crates/unicode-segmentation
[54]: https://crates.io/crates/argh
[55]: https://crates.io/crates/argh_derive
[56]: https://crates.io/crates/argh_shared
[58]: https://crates.io/crates/heck
[18]: https://crates.io/crates/gumdrop

Todo
----

- [ ] Test fixtures
- [ ] Http source
- [ ] Research other popular rust VCSs, add them.
- [ ] Output formats such as JSON
