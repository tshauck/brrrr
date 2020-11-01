FROM rust:latest

WORKDIR /usr/src/brrrr

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/brrrr"]
