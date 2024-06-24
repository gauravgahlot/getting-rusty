# wc-rs

`wc-rs` is a CLI built using the [clap][1] crate.
The motivation to build this project came from [the Challenge - Building wc][2].

## Usage

### Installation

You can install the CLI using the below command:

```sh
cargo install --path .
```

### Help

By using the `--help` option you can obtain the usage help for the CLI:

```sh
➜ wc-rs --help
The wc-rs utility displays the count of lines, words, characters, and bytes contained in each input file

Usage: wc-rs [OPTIONS] [FILES]...

Arguments:
  [FILES]...

Options:
  -c             The number of bytes in each input file is written to the standard output. This will cancel out any prior usage of the -m option
  -l             The number of lines in each input file is written to the standard output
  -w             The number of words in each input file is written to the standard output
  -m             The number of characters in each input file is written to the standard output
  -h, --help     Print help
  -V, --version  Print version
```

### Examples

- Getting details of a single file:

```sh
➜ wc-rs test.txt
        5       5       22      test.txt
      #lines  #words  #bytes
```

- Getting details for multiple files:

```sh
➜ wc-rs -wm test.txt Cargo.toml
        5       16      test.txt
        25      138     Cargo.toml
        30      154     total
```

- Getting details for data from standard input

```sh
➜ wc-rs
data from std input
        1       4       19
```

- Display number of lines and characters only

```sh
➜ wc-rs -lm test.txt
        5       16      test.txt
```

[1]: https://docs.rs/clap/latest/clap/index.html
[2]: https://codingchallenges.fyi/challenges/challenge-wc/
