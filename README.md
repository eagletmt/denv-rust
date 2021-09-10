# denv-rust
[![Build Status](https://github.com/eagletmt/denv-rust/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/eagletmt/denv-rust/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/denv.svg)](https://crates.io/crates/denv)

Load environment variables from .env file.

## CLI usage
```
Usage: denv [OPTIONS] COMMAND...

Options:
    -f, --filename FILENAME
                        Path to .env file
    -h, --help          Print help
```

```
% cat .env
FOO=bar
% denv printenv FOO
bar
% cat hoge.env
HOGE=fuga
% denv -f printenv HOGE
fuga
```

## Library usage
```rust
denv::load("/path/to/.env").expect("Unable to load .env file");
```

## Acknowledgment
Original implementation is written by taiki45.
https://github.com/taiki45/denv
