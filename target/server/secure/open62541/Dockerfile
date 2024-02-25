FROM debian:bullseye

RUN apt update && apt install -y git  cmake python3 wget binutils gcc g++


ENV CXX="g++"
WORKDIR /dependencie
WORKDIR /dependencie/openssl
ARG COMMIT
RUN git -c advice.detachedHead=false clone --branch OpenSSL_1_1_1f --depth 1  https://github.com/openssl/openssl.git .

RUN ./config
RUN make -j
RUN make install_sw 

ARG VersionSUT
WORKDIR /dependencie
RUN git clone -c advice.detachedHead=false --depth 1 --branch $VersionSUT https://github.com/open62541/open62541.git

WORKDIR /dependencie/open62541

RUN  mkdir build && cmake -S . -B build \
     -DUA_ENABLE_ENCRYPTION=OPENSSL \
     -DUA_ENABLE_ENCRYPTION_OPENSSL=ON &&\
     cd build && make -j 4 &&\
     make install 

WORKDIR /server

RUN  cp /dependencie/open62541/examples/common.h common.h &&\
     cp /dependencie/open62541/examples/server_ctt.c server_ctt.c &&\
     gcc -c server_ctt.c -o server_ctt.o  &&\
     gcc server_ctt.o -o server_ctt  -pthread -lopen62541 -lcrypto 
     

WORKDIR /crypto
COPY trustedlist/user_cert_true.der /crypto/trustedlist/user_cert_true.der
COPY trustedlist/uaexpert.der /crypto/trustedlist/uaexpert.der
COPY selfcertificate /crypto/selfcertificate



