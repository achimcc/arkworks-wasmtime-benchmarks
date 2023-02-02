FROM ubuntu:22.04
RUN echo 'APT::Install-Suggests "0";' >> /etc/apt/apt.conf.d/00-docker
RUN echo 'APT::Install-Recommends "0";' >> /etc/apt/apt.conf.d/00-docker
RUN DEBIAN_FRONTEND=noninteractive \
  apt-get update \
  && apt-get install -y python3 \
  && rm -rf /var/lib/apt/lists/*

# RUN rustup component add rust-src rustc-dev llvm-tools-preview 
# RUN cargo +nightly install racer
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
WORKDIR /usr/src/rust-benchmark
RUN apt update --yes
RUN apt install clang lldb lld wget curl --yes
RUN wget https://repo.anaconda.com/archive/Anaconda3-2022.10-Linux-x86_64.sh --no-check-certificate
RUN chmod +x Anaconda3-2022.10-Linux-x86_64.sh
RUN yes yes | ./Anaconda3-2022.10-Linux-x86_64.sh
RUN echo "export PATH=./yes/bin:$PATH" >>  ~/.bashrc 
ENV CONDA ./yes/bin/
ENV PATH="${CONDA}:${PATH}"
RUN ln -s ./yes/bin/conda /usr/local/bin/conda
RUN eval $(conda shell.bash hook)
RUN conda init bash
RUN conda update --all 
RUN conda create -c conda-forge -n rustenv rust
RUN . activate rustenv
ENV PATH="${EMSDK}:${EMSDK}/emscripten/sdk:${EMSDK}/llvm/clang/bin:${EMSDK}/node/current/bin:${EMSDK}/binaryen/bin:${PATH}"
SHELL ["./yes/bin/conda", "run", "-n", "rustenv", "/bin/bash", "-c"]
RUN git clone https://github.com/emscripten-core/emsdk.git
RUN cd emsdk && git pull
RUN chmod +x ./emsdk/emsdk
RUN ./emsdk/emsdk install latest
RUN ./emsdk/emsdk activate latest
RUN chmod +x ./emsdk/emsdk_env.sh
RUN rm /bin/sh && ln -s /bin/bash /bin/sh
RUN cd emsdk && source ./emsdk_env.sh
RUN ./emsdk/emsdk_env.sh
ENV EMSDK ./emsdk
ENV EMSCRIPTEN=${EMSDK}/emscripten/sdk
ENV EM_DATA ${EMSDK}/.data
ENV EM_CONFIG ${EMSDK}/.emscripten
ENV EM_CACHE ${EM_DATA}/cache
ENV EM_PORTS ${EM_DATA}/ports
RUN cd / && find . -name rustup
RUN cd / && find . -name cargo
RUN curl https://sh.rustup.rs -sSf | RUSTUP_INIT_SKIP_PATH_CHECK=yes sh -s -- -y
RUN cd / && find . -name rustup
RUN cd / && find . -name cargo
RUN /bin/bash -c "source /usr/local/cargo/env"
ENV RUST ./usr/local/cargo/env
ENV RUSTUP ./usr/local/cargo/bin/cargo
ENV PATH="${RUSTUP}:${RUST}:${PATH}"
RUN rustup install nightly 
RUN rustup +nightly target add wasm32-wasi 
RUN cargo +nightly build --release --target wasm32-wasi 
RUN cp target/wasm32-wasi/release/bls-381-wasm-benchmark.wasm /benchmark.wasm
# We output the Wasm file to /benchmark.wasm, where the client expects it.
