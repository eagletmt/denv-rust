# denv-rust
[![Build Status](https://travis-ci.org/eagletmt/denv-rust.svg?branch=master)](https://travis-ci.org/eagletmt/denv-rust)

Rust version of [denv](https://github.com/taiki45/denv)

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
