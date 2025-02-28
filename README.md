# cargo-duplicates

A cargo subcommand for displaying when different versions of a same dependency are pulled in.
You can shorten the output by passing `--short` as argument.

## Demo

Once installed, running `cargo duplicates` in a project directory looks like the following:

```text
Package    Versions
-------    --------
hex        0.4.3  0.3.2
rand_core  0.6.3  0.5.1

hex 0.4.3:
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => hex 0.4.3
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => cargo-util 0.1.1 => hex 0.4.3
hex 0.3.2:
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => cargo-util 0.1.1 => crypto-hash 0.3.4 => hex 0.3.2

rand_core 0.6.3:
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => tempfile 3.2.0 => rand 0.8.4 => rand_core 0.6.3
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => cargo-util 0.1.1 => tempfile 3.2.0 => rand 0.8.4 => rand_core
  0.6.3
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => tempfile 3.2.0 => rand 0.8.4 => rand_chacha 0.3.1 => rand_core
  0.6.3
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => cargo-util 0.1.1 => tempfile 3.2.0 => rand 0.8.4 => rand_chacha
  0.3.1 => rand_core 0.6.3
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => tempfile 3.2.0 => rand 0.8.4 => rand_hc 0.3.1 => rand_core 0.6.3
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => cargo-util 0.1.1 => tempfile 3.2.0 => rand 0.8.4 => rand_hc 0.3.1
  => rand_core 0.6.3
rand_core 0.5.1:
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => im-rc 15.0.0 => rand_core 0.5.1
- Because of cargo-duplicates 0.4.0 => cargo 0.57.0 => im-rc 15.0.0 => rand_xoshiro 0.4.0 => rand_core 0.5.1
```
