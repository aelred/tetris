# syntax=docker/dockerfile:1.0-experimental
FROM rustlang/rust:nightly as build
WORKDIR /tmp
RUN USER=root cargo new --bin builder
WORKDIR /tmp/builder
COPY . .
RUN --mount=type=cache,target=../../usr/local/cargo/registry \
    --mount=type=cache,target=../../usr/local/cargo/registry \
    --mount=type=cache,target=target \
    cargo build --release --package tetris-server
RUN --mount=type=cache,target=target cp target/release/tetris-server /tmp/server

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=build /tmp/server /bin
ENTRYPOINT ["server"]