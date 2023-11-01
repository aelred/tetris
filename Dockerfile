FROM rust:1.73.0 as build
RUN cargo install just
WORKDIR /build

FROM build as server-build
COPY . .
RUN just build-server

FROM build as wasm-build
RUN cd / && git clone --depth=1 https://github.com/emscripten-core/emsdk.git
RUN cd /emsdk && ./emsdk install 3.1.43
RUN cd /emsdk && ./emsdk activate 3.1.43
ENV PATH="/emsdk:/emsdk/upstream/emscripten:${PATH}"
COPY justfile .
RUN just init-wasm
COPY . .
RUN just build-wasm

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=server-build /build/target/release/tetris-server /bin
COPY --from=wasm-build /build/static /static
ENV ROCKET_ADDRESS=0.0.0.0
ENV STATIC_FILES /static
EXPOSE 8000
ENTRYPOINT ["tetris-server"]