[![Build Status](https://travis-ci.org/Sh4pe/cbt.svg?branch=master)](https://travis-ci.org/Sh4pe/cbt)

# `cbt` - clipboard transformations

The **c**lip**b**oard **t**ransformer retrieves new values from the system clipboard and runs them through a series of user-defined transformations. 

These transformations can be either inline shell commands like `sed s/a/b/g` or arbitrarily complex shell commands.

## Examples

Print the current clipboard value as soon as it changes.

```
$ cbt
```

Transform `a`s to `A`s and `b`s to `B`s.

```
$ cbt 'sed s/a/A/g' 'sed s/b/B/g'
```

Pretty-print JSON-content in the clipboard using [jq](https://stedolan.github.io/jq/).

```
$ cbt 'jq .'
```

Count how many words are separated by a `,` (works for the macOS version of `sed`).

```
$ cbt "sed 's/,/\'$'\n/g'" "wc -l"
```

If the clipboard content matches "foo", run it through your super-complex Python filter.

```
$ cbt 'grep foo' ./super_complex_filter.py
```

Shoot yourself in the foot with an infinite loop.

```
$ cbt 'xargs echo "haha "' pbcopy
```

## Installation

### Build from source locally

The source install requires that you have the [Rust](https://www.rust-lang.org) compiler installed. Once it is present, run:

```
$ git clone git@github.com:Sh4pe/cbt.git
$ cd cbt
$ cargo install
```

This installs `cbt` to your `$CARGO_HOME/bin` directory.

### Install from crates.io

To build directly from [crates.io](https://crates.io), run:

```
$ cargo install cbt
```