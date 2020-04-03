cargo-lockdiff
==============

[![Crates.io](https://img.shields.io/crates/l/cargo-lockdiff)](https://crates.io/crates/cargo-lockdiff)

See what crates have changed after you run `cargo update` by comparing Cargo.lock to the vcs HEAD or file of your choice.

Supports git, mercurial, and plain files.

Example
-------

### Raw

```
$ cargo lockdiff --from "HEAD@{2 months ago}" --links

| Package                       | From    | To                           |
|-------------------------------|---------|------------------------------|
| [actix-web][12]               | 1.0.9   | 2.0.0                        |
| [chrono][44]                  | 0.4.10  | 0.4.11                       |
| [crossbeam-utils][54]         | 0.7.0   | 0.7.2                        |
| [enum-as-inner][67]           | 0.2.1   | 0.3.2                        |
| [flate2][71]                  | 1.0.13  | 1.0.14                       |
| [lock_api][112]               | 0.3.2   | 0.3.3                        |
| [pin-project-internal][312]   | NEW     | 0.4.8                        |
| [pin-project-lite][313]       | NEW     | 0.1.4                        |
| [proc-macro2][159]            | 1.0.6   | 1.0.9                        |
| [rust-embed-impl][190]        | 5.1.0   | 5.5.1                        |
| [ryu][196]                    | 1.0.2   | 1.0.3                        |
| [tokio-current-thread][230]   | 0.1.6   | 0.1.7                        |
| [tokio-signal][235]           | 0.2.7   | REMOVED                      |
| [tracing-subscriber][331]     | NEW     | 0.2.3                        |
| [tracing][242]                | 0.1.10  | 0.1.13                       |

[12]: https://crates.io/crates/actix-web
[44]: https://crates.io/crates/chrono
[54]: https://crates.io/crates/crossbeam-utils
[67]: https://crates.io/crates/enum-as-inner
[71]: https://crates.io/crates/flate2
[112]: https://crates.io/crates/lock_api
[312]: https://crates.io/crates/pin-project-internal
[313]: https://crates.io/crates/pin-project-lite
[159]: https://crates.io/crates/proc-macro2
[190]: https://crates.io/crates/rust-embed-impl
[196]: https://crates.io/crates/ryu
[230]: https://crates.io/crates/tokio-current-thread
[235]: https://crates.io/crates/tokio-signal
[331]: https://crates.io/crates/tracing-subscriber
[242]: https://crates.io/crates/tracing
```

### Rendered

| Package                       | From    | To                           |
|-------------------------------|---------|------------------------------|
| [tracing][242]                | 0.1.10  | 0.1.13                       |
| [pin-project-lite][313]       | NEW     | 0.1.4                        |
| [tracing-subscriber][331]     | NEW     | 0.2.3                        |
| [flate2][71]                  | 1.0.13  | 1.0.14                       |
| [actix-web][12]               | 1.0.9   | 2.0.0                        |
| [chrono][44]                  | 0.4.10  | 0.4.11                       |
| [enum-as-inner][67]           | 0.2.1   | 0.3.2                        |
| [ryu][196]                    | 1.0.2   | 1.0.3                        |
| [pin-project-internal][312]   | NEW     | 0.4.8                        |
| [tokio-signal][235]           | 0.2.7   | REMOVED                      |
| [proc-macro2][159]            | 1.0.6   | 1.0.9                        |
| [crossbeam-utils][54]         | 0.7.0   | 0.7.2                        |
| [lock_api][112]               | 0.3.2   | 0.3.3                        |
| [rust-embed-impl][190]        | 5.1.0   | 5.5.1                        |
| [tokio-current-thread][230]   | 0.1.6   | 0.1.7                        |

[242]: https://crates.io/crates/tracing
[313]: https://crates.io/crates/pin-project-lite
[331]: https://crates.io/crates/tracing-subscriber
[71]: https://crates.io/crates/flate2
[12]: https://crates.io/crates/actix-web
[44]: https://crates.io/crates/chrono
[67]: https://crates.io/crates/enum-as-inner
[196]: https://crates.io/crates/ryu
[312]: https://crates.io/crates/pin-project-internal
[235]: https://crates.io/crates/tokio-signal
[159]: https://crates.io/crates/proc-macro2
[54]: https://crates.io/crates/crossbeam-utils
[112]: https://crates.io/crates/lock_api
[190]: https://crates.io/crates/rust-embed-impl
[230]: https://crates.io/crates/tokio-current-thread

Install
-------

```bash
cargo install cargo-lockdiff

# try it
cargo lockdiff --help
```

### Dependencies

To use `git` or `hg` the respective binary must be found in `PATH`.


Usage
-----

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

### Options

Environment variables prefixed with `CARGO_LOCKDIFF_`, such as `CARGO_LOCKDIFF_NO_LINKS=true`, may also be used. Set these in your `~/.profile` (specific to your shell, of course) as defaults.

- `-p, --path <path>`: Base to with which to prefix paths. E.g. `-p app` would look for HEAD:app/Cargo.lock and app/Cargo.lock. Env: `CARGO_LOCKDIFF_PATH`
- `--from <fileish>`: The file, vcs ref, or vcs ref with filename to compare from.  To force the use of a particular vcs, prepend it with a colon. E.g. "hg:." Env: `CARGO_LOCKDIFF_FROM`

- `--to <fileish>`: The file, vcs ref, or vcs ref with filename to compare to.  To force the use of a particular vcs, prepend it with a colon. E.g. "hg:." Env: `CARGO_LOCKDIFF_TO`

- `-l, --links`: Include links to where possible. Env: `CARGO_LOCKDIFF_NO_LINKS` ("true" or "false")

- `--help`: Display usage information

Todo
----

- [ ] Handle manifest-path, https://docs.rs/cargo_metadata/0.9.1/cargo_metadata/4
- [ ] Test fixtures
- [ ] Http source
- [ ] Research other popular rust VCSs, add them.
- [ ] Output formats such as JSON
- [ ] "compare versions" links
