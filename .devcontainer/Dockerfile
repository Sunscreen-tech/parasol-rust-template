FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

RUN sudo apt-get update && sudo apt-get install -y build-essential cmake clang git
RUN cargo install --git https://github.com/Sunscreen-tech/foundry --locked --profile local forge cast anvil
