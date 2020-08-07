# Rug (Random Username Generator)

Generates random usernames from a given word list.

```text
$ rug
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
-l, --list <list-file-path>       Provide a txt file with a list of words to generate username from randomly                                  
-m, --maximum <maximum-length>    Set maximum username length. Must be greater than 5 [default: 100]                                          
-n, --number <number-to-print>    Set how many random usernames to output [default: 10]   
```

If the `maximum` option is set less than 11, rug will only pull one word from the word list. If it's set at 11 or more, it will pull two words from the word list.

**Note**: Do NOT use this program to create passwords. 

## Usage examples

- `rug` generates 10 random usernames from included word list (see below)
- `rug -l path/to/a/custom_wordlist.txt` generates 10 random usernames using words from provided `.txt` file, where each word is on its own line
- `rug -n 5 -m 12` generates 5 random usernames with a maximum length of 12 characters.
>>>>>>> 62011e9cb9745497a41711662d8849c6b43409b5

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/rug`

## Note on the included word lists

If no word list is provided, rug uses a 16,098-word list adapted from [an AgileBits word list](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt). 

A copy of the [EFF long word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt) (7,776 words) is included in the repo as well. Of course you can use your word list with the `--words` option.
