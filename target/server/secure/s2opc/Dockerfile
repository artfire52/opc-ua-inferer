FROM debian:bullseye

RUN apt update && apt install -y git  cmake python3  binutils libexpat1-dev g++ libmbedtls-dev wget autoconf bsdmainutils

WORKDIR /check

RUN wget https://github.com/libcheck/check/releases/download/0.14.0/check-0.14.0.tar.gz &&\
    tar -xvf check-0.14.0.tar.gz &&\
    cd check-0.14.0 &&\
    ./configure &&\
    cmake -B build -S . -DHAVE_SUBUNIT=0 &&\
    cd build && make &&\
    make install
RUN git clone -c advice.detachedHead=false https://github.com/libexpat/libexpat.git && cd libexpat/expat &&\
    cmake -B build -S . -DEXPAT_BUILD_PKGCONFIG:BOOL=ON &&\
    cd build && make &&\
    make install 

WORKDIR /server
ARG VersionSUT
RUN git clone -c advice.detachedHead=false --depth 1 --branch $VersionSUT https://gitlab.com/systerel/S2OPC.git 
WORKDIR /server/S2OPC
RUN rm -rf build
RUN sed -i "s/4841/4840/g" tests/ClientServer/validation_tests/server/toolkit_test_server.c
RUN sed -i "79i\"issued/uaexpert.der\",\n" tests/ClientServer/validation_tests/server/toolkit_test_server.c
RUN sed -i "79i\"issued/user_cert_true.der\",\n" tests/ClientServer/validation_tests/server/toolkit_test_server.c
RUN sed -i "s/-aes-256-cbc//" samples/ClientServer/data/cert/generate_certs.sh
RUN cd samples/ClientServer/data/cert/ && ./generate_certs.sh >/dev/null
RUN sed -i "s/(default_server_cert, default_key_cert, true);/(default_server_cert, default_key_cert,false);/" tests/ClientServer/validation_tests/server/toolkit_test_server.c
RUN SECURITY_HARDENING=1 POSITION_INDEPENDENT_EXECUTABLE=1 USE_STATIC_EXT_LIBS=0 ENABLE_TESTING=1 S2OPC_CLIENTSERVER_ONLY=1 ./build.sh

WORKDIR /server/S2OPC/build/bin
COPY trustedlist/uaexpert.der issued/uaexpert.der
COPY trustedlist/user_cert_true.der issued/user_cert_true.der