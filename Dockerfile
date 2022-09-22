FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo clean
RUN cargo install --path .
RUN cargo build --release

ENTRYPOINT ["/app/target/release/blockchain-asseter"]