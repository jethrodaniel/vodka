# vodka

Does nothing for now.

Linux only.

## Install

- [rust](https://github.com/rust-lang/rust)
- [autopilot-rs](https://github.com/autopilot-rs/autopilot-rs)
- gtk
- cairo

If you're using [brew](https://brew.sh)

```
brew install cairo
```

```
sudo apt-get install libgtk-3-dev
```

```
sudo apt-get install libgtk-3-dev
```

## Usage

```
vodka start
```

For help, see

```
vodka -h # or --help
```

To enable `info` logging

```
vodka start -v
```

Use `-vv` for `trace` logging.

All logging outputs to stdout and `vodka.log` (for now).

<!-- ## Testing -->

<!-- Due to the tests actually using the X server, we need to make sure we limit the -->
<!-- tests to a single thread, so we have to use -->

<!-- ``` -->
<!-- RUST_TEST_THREADS=1 cargo test -->
<!-- ``` -->

<!-- or -->

<!-- ``` -->
<!-- cargo test -- --test-threads 1 -->
<!-- ``` -->

<!-- or add it to your shell initalization (e.g. `~/.bashrc`), or whatever. -->

