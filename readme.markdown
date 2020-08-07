# Rug (Random Username Generator)

Generates random usernames from a given word list.

```text
$ rug
barbecuedreliance655
unfathomableguilt521
ace_conduit879
ongoinguser28
simianratio936
phantomsuspect347
insecticidal-glamorization152
carcinogenic-hydroponics23
incompatiblecontingent628
vapid_protea736
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

**Note**: This program is NOT intended to be used to create secure passwords. **Do NOT use this program to create passwords.**

## Usage examples

- `rug` generates 10 random usernames from included word lists (see below)
- `rug -l path/to/a/custom_wordlist.txt` generates 10 random usernames using words from provided `.txt` file, where each word is on its own line
- `rug -n 5 -m 12` generates 5 random usernames with a maximum length of 12 characters.

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/rug`

## On the included word lists

If no word list is provided, rug uses one or two word lists from [the SecureDrop project](https://github.com/freedomofpress/securedrop/) to create usernames. If the maximum length allows (or is not specified), usernames will be in "[adjective](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/adjectives.txt) + [noun](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/nouns.txt) + number" format. If the maximum length is set below 11 characters, rug will use a "noun + number" format.

If the user provides a word list of their own, using the `-l` option, rug will use that provided list to generate _both_ words. 
