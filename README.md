# promptconv
> A simple BASH > Zsh prompt conversion utility.

### What is it?
Just converts a `Bash` shell prompt to a `zsh` prompt.

### Usage
```
$ promptconv --help

promptconv 0.1.1
A simple Bash > Zsh prompt converter.

USAGE:
    promptconv [FLAGS] <prompt>

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Print less text, only printing the zsh prompt when done
    -V, --version    Prints version information

ARGS:
    <prompt>    Bash prompt to convert
```

### Warnings
This program cannot:
* Filter out Bash-specific escapes

It is meant to convert most Bash escapes to Zsh escapes.

### Installation
If you're interested, you can install `promptconv` through one of the following means:
* The releases page (only contains Linux binaries)
* Cargo (`cargo install promptconv`)

<h3 align="center">Additional Links</h3>
<h6 align="center"><a href="./LICENSE">License</a> | <a href="https://github.com/notronaldmcdonald/promptconv/releases">Releases</a> | <a href="https://crates.io/crates/promptconv">crates.io</a></h6>
