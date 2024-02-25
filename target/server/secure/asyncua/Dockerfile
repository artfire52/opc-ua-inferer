FROM python:3.8

ARG VersionSUT
RUN git clone -c advice.detachedHead=false --depth 1 --branch $VersionSUT https://github.com/FreeOpcUa/opcua-asyncio.git opcua  && pip install asyncua==$VersionSUT
WORKDIR /opcua/examples
COPY trustedlist/user_cert_true.der ./user_cert_true.der
RUN sed -i "s/certificates\/peer-certificate-example-1.der/user_cert_true.der/g"  server-with-encryption.py

