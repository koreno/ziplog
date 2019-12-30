# Ziplog

A sorted log file merger using timestamps, written in Rust. Based on an earlier [Python version](https://github.com/weka-io/easypy/blob/master/easypy/ziplog.py) by [koreno](http://github.com/koreno).

## Building

Install the [Rust](https://www.rust-lang.org/) toolchain, and do:

```
cargo build --release
```

Output is expected at `target/release/ziplog`.

## Running

```
ZipLog - merge logs by timestamps

USAGE:
    ziplog [OPTIONS] [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interval <interval>                 Show interval by seconds (s), or milliseconds (ms)
    -p, --prefix <prefix>                     The default prefix to prepend to timestamped lines [default: > ]
    -f, --prefixed-file <prefixed-logs>...    Prefixed log files, using a different prefix for each timestamped file

ARGS:
    <FILE>...    Log files; Use "-" for STDIN
```
