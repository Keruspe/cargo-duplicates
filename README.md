# cargo-duplicates

A cargo subcommand for displaying when different versions of a same dependency are pulled in.

## Demo

Once installed, running `cargo duplicates` in a project directory looks like the following:

```text
$ cargo duplicates
Package    Versions
-------    --------
cfg-if     1.0.0  0.1.10
hex        0.4.2  0.3.2
humantime  2.0.1  1.3.0

cfg-if 1.0.0:
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => flate2 1.0.19 => crc32fast 1.2.1 => cfg-if 1.0.0
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => filetime 0.2.13 => cfg-if 1.0.0
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => tar 0.4.30 => filetime 0.2.13 => cfg-if 1.0.0
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => flate2 1.0.19 => cfg-if 1.0.0
cfg-if 0.1.10:
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => crossbeam-utils 0.7.2 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => ignore 0.4.16 => crossbeam-utils 0.7.2 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => tempfile 3.1.0 => rand 0.7.3 => getrandom 0.1.15 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => im-rc 15.0.0 => rand_core 0.5.1 => getrandom 0.1.15 => cfg-if
  0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => tempfile 3.1.0 => rand 0.7.3 => rand_core 0.5.1 => getrandom
  0.1.15 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => tempfile 3.1.0 => rand 0.7.3 => rand_chacha 0.2.2 => rand_core
  0.5.1 => getrandom 0.1.15 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => tempfile 3.1.0 => rand 0.7.3 => rand_hc 0.2.0 => rand_core 0.5.1
  => getrandom 0.1.15 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => im-rc 15.0.0 => rand_xoshiro 0.4.0 => rand_core 0.5.1 =>
  getrandom 0.1.15 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => log 0.4.11 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => env_logger 0.7.1 => log 0.4.11 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => git2 0.13.12 => log 0.4.11 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => git2-curl 0.14.1 => git2 0.13.12 => log 0.4.11 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => git2-curl 0.14.1 => log 0.4.11 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => ignore 0.4.16 => globset 0.4.6 => log 0.4.11 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => ignore 0.4.16 => log 0.4.11 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => rustfix 0.5.1 => log 0.4.11 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => crypto-hash 0.3.4 => openssl 0.10.30 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => curl 0.4.34 => socket2 0.3.15 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => crates-io 0.31.1 => curl 0.4.34 => socket2 0.3.15 => cfg-if
  0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => git2-curl 0.14.1 => curl 0.4.34 => socket2 0.3.15 => cfg-if
  0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => miow 0.3.5 => socket2 0.3.15 => cfg-if 0.1.10
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => tempfile 3.1.0 => cfg-if 0.1.10

hex 0.4.2:
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => hex 0.4.2
hex 0.3.2:
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => crypto-hash 0.3.4 => hex 0.3.2

humantime 2.0.1:
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => humantime 2.0.1
humantime 1.3.0:
- Because of cargo-duplicates 0.2.1 => cargo 0.48.0 => env_logger 0.7.1 => humantime 1.3.0
```
