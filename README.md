# ruffle (fork)

Personal fork of [Ruffle](https://github.com/ruffle-rs/ruffle) — an Adobe Flash
Player emulator written in Rust.

- **Upstream**: https://github.com/ruffle-rs/ruffle (remote: `source`)
- **This fork**: https://github.com/manhnv198669/ruffle (remote: `origin`)

Original Ruffle README preserved at [`OLD_README.md`](OLD_README.md). All credit
for the project, its design, and 99.99% of its code belongs to the upstream
Ruffle contributors. See [`LICENSE.md`](LICENSE.md) for license terms (MIT /
Apache-2.0).

## Why a fork

Holds a small number of local patches that aren't intended for upstream and
shouldn't live as orphan diffs on a laptop. The default branch tracks upstream
`master` with patches rebased on top.

## Sync with upstream

```bash
git fetch source
git rebase source/master
# resolve conflicts if any
git push --force-with-lease origin master
```

## Build

Same as upstream. See [`OLD_README.md`](OLD_README.md) for the full build
instructions. Quick desktop build:

```bash
cargo build --release -p ruffle_desktop
# binary at: target/release/ruffle_desktop
```
