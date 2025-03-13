Fuzzing with [cargo-fuzz](https://rust-fuzz.github.io/book/introduction.html).

```sh
# Fuzz dispatching key bindings with random key inputs
cargo +nightly fuzz run dispatch

# Fuzz parsing a key sequence like "Ctrl+X"
cargo +nightly fuzz run parse
```
