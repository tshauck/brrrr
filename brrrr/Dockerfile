FROM rust:latest

WORKDIR /usr/src/brrrr

COPY . .

RUN cargo build --release

RUN cargo install --path ./brrrr

ENTRYPOINT ["/usr/local/cargo/bin/brrrr"]
