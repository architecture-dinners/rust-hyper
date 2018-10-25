# see http://whitfin.io/speeding-up-rust-docker-builds
FROM rust:1.29-stretch as build

RUN USER=root cargo new --bin rust-hyper
WORKDIR /rust-hyper

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release

# our final base
FROM rust:1.29-stretch

# copy the build artifact from the build stage
COPY --from=build /rust-hyper/target/release/rust-hyper .

# setup the environment
ENV RUST_LOG rust_hyper=info
ENV RUST_BACKTRACE=1

ENV NOM_SERVER_PORT 8085
ENV NOM_SERVER_HOST 0.0.0.0

# set the startup command to run your binary
CMD ["./rust-hyper"]
