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

hex 0.4.2:
- Because of cargo-duplicates 0.1.0 => cargo 0.48.0 => hex 0.4.2
hex 0.3.2:
- Because of cargo-duplicates 0.1.0 => cargo 0.48.0 => crypto-hash 0.3.4 => hex 0.3.2

humantime 2.0.1:
- Because of cargo-duplicates 0.1.0 => cargo 0.48.0 => humantime 2.0.1
humantime 1.3.0:
- Because of cargo-duplicates 0.1.0 => cargo 0.48.0 => env_logger 0.7.1 => humantime 1.3.0

cfg-if 1.0.0:
- Because of cargo-duplicates 0.1.0 => cargo 0.48.0 => flate2 1.0.19 => crc32fast 1.2.1 => cfg-if 1.0.0
cfg-if 0.1.10:
- Because of cargo-duplicates 0.1.0 => cargo 0.48.0 => crossbeam-utils 0.7.2 => cfg-if 0.1.10
```
