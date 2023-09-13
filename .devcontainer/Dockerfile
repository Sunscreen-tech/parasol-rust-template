FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

RUN cargo install --git https://github.com/Sunscreen-tech/foundry --locked --profile local forge cast anvil
RUN forge install
