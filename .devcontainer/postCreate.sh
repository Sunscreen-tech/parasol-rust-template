#!/usr/bin/env bash

# NOTE: We are abusing a race condition here to get the some
# initial bash commands related to environmental variables set. The
# postCreateCommand is run and then the container is connected to. Any
# short running process will happen before the editor is launched, meaning
# we can set a custom cargo and a custom rustup home before the editor
# is launched.

# Blank out the first message
read -r -d '' STARTUP_MESSAGE <<'EOF'
Welcome to the Parasol testnet! You can create new account credentials by running 

cast wallet new

and get funds at https://faucet.sunscreen.tech.

Documentation can be found at https://hackathon.sunscreen.tech.
EOF

# Do not print out an initial message.
sudo echo "" > /workspaces/.codespaces/shared/first-run-notice.txt

# and instead always print out this information
echo "$STARTUP_MESSAGE" >> ~/.msg
echo "cat ~/.msg" >> ~/.bashrc

# Install rust for the current user
export RUSTUP_HOME=~/.rustup
export CARGO_HOME=~/.cargo
export RUSTUP_INIT_SKIP_PATH_CHECK=yes

echo "RUSTUP_HOME=$RUSTUP_HOME" >> ~/.bashrc
echo "CARGO_HOME=$CARGO_HOME" >> ~/.bashrc

# Force the installation of the git submodules
git remote add origin https://github.com/Sunscreen-tech/parasol-rust-template.git
git branch --set-upstream-to=origin/main
git config pull.rebase true
git pull
git submodule update --init

forge install --root=./contracts

# Remove the origin since we do not need it anymore
git remote remove origin

# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y