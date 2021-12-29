<div align="center">
<h1>git-smart-checkout</h1>

<p>
A git command extension for switching git branches more efficiently.
</p>

![brain](https://user-images.githubusercontent.com/27342306/147604624-9d62c8ca-d2f2-42a3-973e-4341a09b0d23.png)
<br>

[![Crates.io](https://img.shields.io/crates/v/git-smart-checkout.svg)](https://crates.io/crates/git-smart-checkout)
[![License](https://img.shields.io/crates/l/git-smart-checkout.svg)](./LICENSE)

![Demo](https://raw.githubusercontent.com/craciuncezar/git-smart-checkout/main/.github/images/demo.gif)

</div>

## About

Interactively switch branches or fuzzy search for that forgotten branch name.

All powered by the speed ⚡️ of rust 🦀.

## Installation

If you have [Rust installed](https://www.rust-lang.org/tools/install) (using the recommended rustup installation method) then you can install the binary from the [crate](https://crates.io/crates/git-smart-checkout) using cargo:

```sh
cargo install git-smart-checkout
```

For Homebrew users, you can install the binary using the following command:

```sh
brew tap craciuncezar/tap
brew install git-smart-checkout
```

You can also install the binary directly from GitHub Releases

```sh
curl -sSL https://github.com/craciuncezar/git-smart-checkout/releases/download/v0.1.0/git-smart-checkout -o /usr/local/bin/git-smart-checkout && chmod +x /usr/local/bin/git-smart-checkout
```

### Git alias

To save typing time you can use a regular git alias for `git smart-checkout`. The following command will add the alias `git sc` to your git config, however feel free to use whatever works best for you:

```sh
git config --global alias.sc smart-checkout
```
