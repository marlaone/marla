FROM debian:latest

WORKDIR /app

RUN apt-get update
RUN apt-get install -y curl build-essential openssl libssl-dev pkg-config

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | RUSTUP_HOME=/opt/rust CARGO_HOME=/opt/rust sh -s -- --no-modify-path -y --default-toolchain=stable

ENV PATH="${PATH}:/opt/rust/bin"

RUN rustup default stable

COPY ./ ./

RUN rm -f ./target/release/deps/marla_*
RUN cargo build --release

EXPOSE 1808
EXPOSE 1809

ENTRYPOINT ["/app/target/release/website"]