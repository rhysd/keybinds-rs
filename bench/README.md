How to run benchmarks:

```sh
# Run all benchmarks
cargo bench -p keybinds-bench

# Only run benchmarks related to dispatching actions
cargo bench -p keybinds-bench -- dispatch::
```
