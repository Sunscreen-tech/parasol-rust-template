#!/usr/bin/env bash

cargo install cargo-watch 
sudo chown -R vscode:vscode /usr/local/cargo/registry
sudo chown -R vscode:vscode /usr/local/cargo/git

# Force the installation of the git submodules
git remote add origin https://github.com/Sunscreen-tech/parasol-rust-template.git
git branch --set-upstream-to=origin/main
git config pull.rebase true
git pull
git submodule update --init
