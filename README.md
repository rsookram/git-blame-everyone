# git-blame-everyone

CLI tool to see total lines last touched, broken out by author.


## Usage

Call `git-blame-everyone` from the command line and pass the paths of the files to blame as
arguments.

For example, if you want to blame all the `*.rs` files in a git repository, you can run:

```shell
git ls-files '*.rs' | xargs git-blame-everyone
```


## Install

Currently, pre-compiled binaries of git-blame-everyone aren't being distributed. You can install it
with [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) by running

```
cargo install --git https://github.com/rsookram/git-blame-everyone
```


## Build

git-blame-everyone can be built from source by cloning this repository and using Cargo.

```
git clone https://github.com/rsookram/git-blame-everyone
cd git-blame-everyone
cargo build --release
```


## Dependencies

The following dependencies are used to implement git-blame-everyone:

- [`anyhow`](https://crates.io/crates/anyhow) to simplify error handling
- [`rayon`](https://crates.io/crates/rayon) to process input files in parallel
