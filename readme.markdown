# Rug (Random Username Generator)

Generates random usernames from a given word list.

```text
flyleaf-therapy41
driveway_rancour983
dignifypartial61
tuberfodder847
thailand-cortices910
doom_digress486
ironwoodtremolo496
bethesdatoad905
athenian_amuse850
ermine-forging601
```

## Usage

```text
USAGE:
    rug [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints verbose output, including parameters as received

OPTIONS:
    -c, --count <count>                  Set how many random usernames to output [default: 10]
    -m, --maximum <maximum-length>       Set maximum username length. Must be greater than 5 [default: 100]
    -w, --words <word-list-file-path>    Provide a txt file with words to generate username from randomly

```

If the `maximum` option is set less than 11, rug will only pull one word from the word list. If it's set at 11 or more, it will pull two words from the word list.

## Examples

- `rug` generates 10 random usernames from included word list (see below)
- `rug -w path/to/a/custom_wordlist.txt` generates 10 random usernames using words from provide file, with each word on its own line
- `rug -c 5 -m 12` generates 5 random usernames with a maximum length of 12 characters.

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/rug`

## Note on the included word lists

If no word list is provided, rug uses is a list adapted from [an AgileBits word list](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt) for generating passwords. (The [EFF long word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt) is included in the repo as well for your convenience.)
