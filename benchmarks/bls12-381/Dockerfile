FROM ubuntu:22.04
RUN echo 'APT::Install-Suggests "0";' >> /etc/apt/apt.conf.d/00-docker
RUN echo 'APT::Install-Recommends "0";' >> /etc/apt/apt.conf.d/00-docker
RUN DEBIAN_FRONTEND=noninteractive \
  apt-get update \
  && apt-get install -y python3 \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
WORKDIR /usr/src/rust-benchmark
RUN apt update --yes
RUN apt install clang lldb lld wget curl git xz-utils bzip2 --yes
RUN apt-get install --reinstall ca-certificates --yes
RUN apt-get install libgl1-mesa-glx libegl1-mesa libxrandr2 libxrandr2 libxss1 libxcursor1 libxcomposite1 libasound2 libxi6 libxtst6 -y
RUN mkdir /usr/local/share/ca-certificates/cacert.org
RUN wget -P /usr/local/share/ca-certificates/cacert.org http://www.cacert.org/certs/root.crt http://www.cacert.org/certs/class3.crt
RUN update-ca-certificates
RUN git config --global http.sslCAinfo /etc/ssl/certs/ca-certificates.crt
RUN wget https://repo.anaconda.com/archive/Anaconda3-2022.10-Linux-x86_64.sh --no-check-certificate
RUN cd / && find . -name cargo
RUN chmod +x Anaconda3-2022.10-Linux-x86_64.sh
RUN yes yes | ./Anaconda3-2022.10-Linux-x86_64.sh
RUN rm Anaconda3-2022.10-Linux-x86_64.sh
RUN echo "export PATH=./yes/bin:$PATH" >>  ~/.bashrc 
ENV CONDA ./yes/bin/
ENV PATH="${CONDA}:${PATH}"
RUN ln -s ./yes/bin/conda /usr/local/bin/conda
RUN eval $(conda shell.bash hook)
RUN conda init bash
RUN conda update --all 
RUN cd / && find . -name cargo
RUN conda create -c conda-forge -n rustenv rust
RUN activate rustenv
SHELL ["./yes/bin/conda", "run", "-n", "rustenv", "/bin/bash", "-c"]
RUN rustc --version
ENV GIT_SSL_NO_VERIFY=1
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
ENV PATH="${EMSDK}:${EMSDK}/emscripten/sdk:${EMSDK}/llvm/clang/bin:${EMSDK}/node/current/bin:${EMSDK}/binaryen/bin:${PATH}"
RUN curl https://sh.rustup.rs -ksSf | sh -s --  -y
RUN chmod +x  $HOME/.cargo/env
RUN $HOME/.cargo/env
ENV RUST ~/.cargo/bin
ENV PATH="${RUST}:${PATH}"
RUN rustup default nightly
RUN rustup target add wasm32-wasi --toolchain nightly
RUN ./yes/envs/rustenv/bin/cargo build --release --target wasm32-wasi 
RUN cp target/wasm32-wasi/release/bls-381-wasm-benchmark.wasm /benchmark.wasm
# We output the Wasm file to /benchmark.wasm, where the client expects it.
