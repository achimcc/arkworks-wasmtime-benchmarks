FROM rust:1.66

RUN rustup install nightly
RUN rustup target add wasm32-wasi --toolchain nightly
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
WORKDIR /usr/src/rust-benchmark
RUN cargo +nightly build --release --target wasm32-wasi
RUN cp target/wasm32-wasi/release/arkworks.wasm /benchmark.wasm
# We output the Wasm file to /benchmark.wasm, where the client expects it.