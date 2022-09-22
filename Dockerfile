FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo clean
RUN cargo install --path .
RUN cargo build --release

EXPOSE 8080
EXPOSE 8081

ENTRYPOINT ["/app/target/release/blockchain-asseter"]