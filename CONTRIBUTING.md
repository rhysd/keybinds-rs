## Bug reporting and feature request

Please create a new issue on GitHub.

https://github.com/rhysd/keybinds-rs/issues/new

## Submitting a patch

Please create a pull request on GitHub. Please ensure that all tests and checks are passing on your local machine before
submitting it.

https://github.com/rhysd/keybinds-rs/pulls

## Commands for development

- `cargo clippy-all` checks all codes in this repository with clippy.
- `cargo test-all` runs all tests.
- `cargo fmt --check` checks the code formatting with rustfmt.
- `cargo bench -p keybinds-bench` runs the [benchmarks](./bench).
- `cargo +nightly fuzz run` runs the [fuzz tests](./fuzz/README.md).

Setting the [`pre-push` hook](./.git-hooks/pre-push) is useful to run all checks on `git push`:

```sh
git config core.hooksPath .git-hooks
```

## Steps to create a new release

1. Determine the next version following the [versioning document](./doc/versioning.md).
2. Confirm all tests and checks pass.
3. Create a new Git tag and push it. (e.g. `git tag v0.1.2 && git push origin v0.1.2`).
4. Write up the release note at the [releases page](https://github.com/rhysd/keybinds-rs/releases) on GitHub.
5. Update the [changelog](./CHANGELOG.md) with `changelog-from-release -p > CHANGELOG.md` using [changelog-from-release](https://github.com/rhysd/changelog-from-release)
   and commit it.
6. Publish the release to [crates.io](https://crates.io/crates/keybinds) with `cargo publish`.

## Generate documents locally

Nightly toolchain is necessary to build the API document the same as the [one on docs.rs](https://docs.rs/keybinds/latest/keybinds/).

```sh
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --no-deps -Z rustdoc-map -Z rustdoc-scrape-examples --all-features --open
```
