# cargo-duplicates

A cargo subcommand for displaying when different versions of a same dependency are pulled in.

## Demo

Once installed, running `cargo duplicates` in a project directory looks like the following:

```text
$ cargo duplicates
Package    Versions
-------    --------
hex        0.4.2  0.3.2
humantime  2.0.1  1.3.0
cfg-if     1.0.0  0.1.10
```
