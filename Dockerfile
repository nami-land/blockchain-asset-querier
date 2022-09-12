FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo install --path .

RUN cargo build --release

ENTRYPOINT ["/app/target/release/neco-blockchain-helper-service"]