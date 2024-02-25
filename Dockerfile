FROM debian:bullseye
RUN apt update && apt install -y python3 git binutils make gcc curl pkg-config 
WORKDIR /
COPY mapper /mapper
COPY learner /learner
WORKDIR /openssl
RUN git -c advice.detachedHead=false clone --branch OpenSSL_1_1_1f --depth 1 https://github.com/openssl/openssl.git .

RUN ./config
RUN make -j 8
RUN make install_sw
RUN ldconfig
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /learner
RUN ./compile_mapper
RUN mkdir result
