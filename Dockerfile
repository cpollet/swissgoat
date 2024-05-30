# Credit: https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

FROM rust as build

# create a new empty shell project
RUN USER=root cargo new --bin swissgoat
WORKDIR /swissgoat

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./build.rs ./build.rs

# build for release
RUN rm ./target/release/deps/swissgoat*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim

# copy the build artifact from the build stage
COPY --from=build /swissgoat/target/release/swissgoat .

EXPOSE 3000

# set the startup command to run your binary
CMD ["./swissgoat"]
