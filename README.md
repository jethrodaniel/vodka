# vodka

Does nothing for now.

## Contents

- [Install](#install)
- [Requirements](#requirements)
- [Usage](#usage)
- [Development](#development)
  - [Testing](#testing)

## Install

```
cargo install vodka
```

## Requirements

- [rust](https://github.com/rust-lang/rust)
- [gtk](https://gtk-rs.org/)

#### rust

#### gtk

https://gtk-rs.org/docs-src/requirements.html

If you're using [brew](https://brew.sh)

```
brew install gtk+3
```

Or apt

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

## Development

### Testing

```
cargo test
```
