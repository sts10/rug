# Rug (Random Username Generator)

Generates random usernames from a given word list.

## Example

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
    -w, --words <word-list-file-path>    Provide file with words to generate username from randomly [default: word-
                                         lists/agile_words.txt]

```

## On the words used

By default, the word list the program uses is a list adapted from [an AgileBits word list](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt) for generating passwords. The [EFF long word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt) is included in the repo as well for your convenience.
