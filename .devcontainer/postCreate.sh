#!/usr/bin/env bash

# Install rust for the current user
export RUSTUP_HOME=~/.rustup
export CARGO_HOME=~/.cargo
export RUSTUP_INIT_SKIP_PATH_CHECK=yes
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source "$HOME/.cargo/env"

# Force the installation of the git submodules
git remote add origin https://github.com/Sunscreen-tech/parasol-rust-template.git
git branch --set-upstream-to=origin/main
git config pull.rebase true
git pull
git submodule update --init
