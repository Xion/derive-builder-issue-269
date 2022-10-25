# derive_builder issue #269

```
$ cargo run

Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/derive-builder-issue-269`
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `42.0`,
 right: `0.0`', src/main.rs:15:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
