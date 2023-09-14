#!/usr/bin/env bash

sudo chown vscode:vscode /usr/local/cargo/registry
cargo install cargo-watch 

# Force the installation of the git submodules
git remote add origin https://github.com/Sunscreen-tech/parasol-rust-template.git
git branch --set-upstream-to=origin/main
git config pull.rebase true
git pull
git submodule update --init
