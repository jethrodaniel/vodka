# vodka

Does nothing for now.

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

## Testing

Due to the tests actually using the X server, we need to make sure we limit the
tests to a single thread, so we have to use

```
RUST_TEST_THREADS=1 cargo test
```

or

```
cargo test -- --test-threads 1
```

or add it to your shell initalization (e.g. `~/.bashrc`), or whatever.
