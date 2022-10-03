#stage 1 - generate a recipe file for dependencies
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

#stage 2 - build dependencies
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

#stage 3 - build the app
FROM rust as builder

#copy the app into the docker image
COPY . /app

# set the working directory
WORKDIR /app

#copy the dependencies from the cacher stage
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

#build the app
RUN cargo build --release

# use google's distroless image as the base image
FROM gcr.io/distroless/cc-debian11

# copy the binary from the builder stage
COPY --from=builder /app/target/release/blockchain-asset-observer /usr/local/bin/blockchain-asset-observer

EXPOSE 8080
EXPOSE 8081

# run the binary
CMD ["blockchain-asset-querier"]
