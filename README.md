cargo-lockdiff
==============

[![Crates.io](https://img.shields.io/crates/v/cargo-lockdiff)](https://crates.io/crates/cargo-lockdiff)

See what crates have changed after you run `cargo update` by comparing Cargo.lock to the vcs HEAD or file of your choice.

Supports git, mercurial, and plain files.

Example
-------

### Raw

```
$ cargo lockdiff --from "HEAD@{2 months ago}"

| Package                      | From    | To                           | Compare    |
|------------------------------|---------|------------------------------|------------|
| [serde_json][C6]             | 1.0.42  | 1.0.48                       | [...][C7]  |
| [sha2][C8]                   | 0.8.0   | 0.8.1                        | [...][C9]  |
| [sharded-slab][CA]           | NEW     | 0.0.8                        |            |
| signal-hook                  | 0.1.12  | REMOVED                      |            |
| [smallvec][CB]               | 1.0.0   | 1.2.0                        | [...][CC]  |
| spin                         | 0.5.2   | REMOVED                      |            |
| string                       | 0.2.1   | REMOVED                      |            |
| [strum][CD]                  | NEW     | 0.18.0                       |            |
| [strum_macros][CE]           | NEW     | 0.18.0                       |            |
| [syn][CF]                    | 1.0.11  | 1.0.17                       | [...][D0]  |
| [thiserror][D2]              | NEW     | 1.0.13                       |            |
| [thiserror-impl][D3]         | NEW     | 1.0.13                       |            |
| [thread_local][D4]           | NEW     | 1.0.1                        |            |
| [tokio][D5]                  | NEW     | 0.2.13                       |            |
| [tokio-codec][D6]            | 0.1.1   | 0.1.2                        | [...][D7]  |
| tokio-signal                 | 0.2.7   | REMOVED                      |            |
| [tracing][F1]                | 0.1.10  | 0.1.13                       | [...][F2]  |
| [tracing-attributes][F3]     | 0.1.5   | 0.1.7                        | [...][F4]  |

[C6]: https://crates.io/crates/serde_json
[C7]: https://diff.rs/serde%5Fjson/1%2E0%2E42/serde%5Fjson/1%2E0%2E48/Cargo.toml
[C8]: https://crates.io/crates/sha2
[C9]: https://diff.rs/sha2/0%2E8%2E0/sha2/0%2E8%2E1/Cargo.toml
[CA]: https://crates.io/crates/sharded-slab
[CB]: https://crates.io/crates/smallvec
[CC]: https://diff.rs/smallvec/1%2E0%2E0/smallvec/1%2E2%2E0/Cargo.toml
[CD]: https://crates.io/crates/strum
[CE]: https://crates.io/crates/strum_macros
[CF]: https://crates.io/crates/syn
[D0]: https://diff.rs/syn/1%2E0%2E11/syn/1%2E0%2E17/Cargo.toml
[D2]: https://crates.io/crates/thiserror
[D3]: https://crates.io/crates/thiserror-impl
[D4]: https://crates.io/crates/thread_local
[D5]: https://crates.io/crates/tokio
[D6]: https://crates.io/crates/tokio-codec
[D7]: https://diff.rs/tokio%2Dcodec/0%2E1%2E1/tokio%2Dcodec/0%2E1%2E2/Cargo.toml
[F1]: https://crates.io/crates/tracing
[F2]: https://diff.rs/tracing/0%2E1%2E10/tracing/0%2E1%2E13/Cargo.toml
[F3]: https://crates.io/crates/tracing-attributes
[F4]: https://diff.rs/tracing%2Dattributes/0%2E1%2E5/tracing%2Dattributes/0%2E1%2E7/Cargo.toml
```

### Rendered

| Package                      | From    | To                           | Compare    |
|------------------------------|---------|------------------------------|------------|
| [serde_json][C6]             | 1.0.42  | 1.0.48                       | [...][C7]  |
| [sha2][C8]                   | 0.8.0   | 0.8.1                        | [...][C9]  |
| [sharded-slab][CA]           | NEW     | 0.0.8                        |            |
| signal-hook                  | 0.1.12  | REMOVED                      |            |
| [smallvec][CB]               | 1.0.0   | 1.2.0                        | [...][CC]  |
| spin                         | 0.5.2   | REMOVED                      |            |
| string                       | 0.2.1   | REMOVED                      |            |
| [strum][CD]                  | NEW     | 0.18.0                       |            |
| [strum_macros][CE]           | NEW     | 0.18.0                       |            |
| [syn][CF]                    | 1.0.11  | 1.0.17                       | [...][D0]  |
| [thiserror][D2]              | NEW     | 1.0.13                       |            |
| [thiserror-impl][D3]         | NEW     | 1.0.13                       |            |
| [thread_local][D4]           | NEW     | 1.0.1                        |            |
| [tokio][D5]                  | NEW     | 0.2.13                       |            |
| [tokio-codec][D6]            | 0.1.1   | 0.1.2                        | [...][D7]  |
| tokio-signal                 | 0.2.7   | REMOVED                      |            |
| [tracing][F1]                | 0.1.10  | 0.1.13                       | [...][F2]  |
| [tracing-attributes][F3]     | 0.1.5   | 0.1.7                        | [...][F4]  |

[C6]: https://crates.io/crates/serde_json
[C7]: https://diff.rs/serde%5Fjson/1%2E0%2E42/serde%5Fjson/1%2E0%2E48/Cargo.toml
[C8]: https://crates.io/crates/sha2
[C9]: https://diff.rs/sha2/0%2E8%2E0/sha2/0%2E8%2E1/Cargo.toml
[CA]: https://crates.io/crates/sharded-slab
[CB]: https://crates.io/crates/smallvec
[CC]: https://diff.rs/smallvec/1%2E0%2E0/smallvec/1%2E2%2E0/Cargo.toml
[CD]: https://crates.io/crates/strum
[CE]: https://crates.io/crates/strum_macros
[CF]: https://crates.io/crates/syn
[D0]: https://diff.rs/syn/1%2E0%2E11/syn/1%2E0%2E17/Cargo.toml
[D2]: https://crates.io/crates/thiserror
[D3]: https://crates.io/crates/thiserror-impl
[D4]: https://crates.io/crates/thread_local
[D5]: https://crates.io/crates/tokio
[D6]: https://crates.io/crates/tokio-codec
[D7]: https://diff.rs/tokio%2Dcodec/0%2E1%2E1/tokio%2Dcodec/0%2E1%2E2/Cargo.toml
[F1]: https://crates.io/crates/tracing
[F2]: https://diff.rs/tracing/0%2E1%2E10/tracing/0%2E1%2E13/Cargo.toml
[F3]: https://crates.io/crates/tracing-attributes
[F4]: https://diff.rs/tracing%2Dattributes/0%2E1%2E5/tracing%2Dattributes/0%2E1%2E7/Cargo.toml

Install
-------

```bash
cargo install cargo-lockdiff

# try it
cargo lockdiff --help
```

### Dependencies

To use `git` or `hg` the respective binary must be found in `PATH`. `cargo metadata` is needed for links.

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

- `-n, --no-links`: Include links to where possible. Env: `CARGO_LOCKDIFF_NO_LINKS` ("true" or "false")

- `--help`: Display usage information

Compare Urls
------------

Compare urls are best effort. Unfortunately there is no strict mapping between crate version numbers and VCS tags so the URLs are just guesses. There doesn't seem to be much consistency. Many maintainers seem to prefix their version number tags with a "v" or, for larger projects, the names of individual crates. If anyone has ideas on how to solve improve it I'd love to try some out.

Currently only Github is supported. Others will be added.

Todo
----

- [ ] Handle manifest-path, https://docs.rs/cargo_metadata/0.9.1/cargo_metadata/4
- [ ] Test fixtures
- [ ] Http source
- [ ] Research other popular rust VCSs, add them.
- [ ] Output formats such as JSON
