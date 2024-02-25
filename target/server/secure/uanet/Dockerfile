FROM debian:bullseye
RUN apt update && apt install -y wget git
RUN wget https://packages.microsoft.com/config/debian/11/packages-microsoft-prod.deb -O packages-microsoft-prod.deb &&\
  dpkg -i packages-microsoft-prod.deb && \
  rm  packages-microsoft-prod.deb

RUN apt update 
RUN apt-get install -y apt-transport-https dotnet-sdk-6.0 
WORKDIR /Server
ARG VersionSUT
RUN git clone https://github.com/OPCFoundation/UA-.NETStandard-Samples.git .
RUN git checkout 3b061f38dfb32f98c6057e78e817b07a65b2e1b6


WORKDIR Samples/NetCoreConsoleServer
RUN mkdir -p pki/trusted/certs
RUN pwd && ls
RUN sed "47,49d" -i NetCoreConsoleServer.csproj &&\
  sed "61d" -i Opc.Ua.SampleServer.Config.xml &&\
  sed "s/localhost:51210/localhost:4840/g" -i Opc.Ua.SampleServer.Config.xml && \
  sed "s/>.*trusted</>\/Server\/Samples\/NetCoreConsoleServer\/pki\/trusted</"  -i Opc.Ua.SampleServer.Config.xml 
RUN sed "s/Version=\"1.4.369.30\"/Version=$VersionSUT/" -i NetCoreConsoleServer.csproj
COPY trustedlist/* /Server/Samples/NetCoreConsoleServer/pki/trusted/certs/

WORKDIR /Server
RUN dotnet restore 'UA Sample Applications.sln' 
WORKDIR Samples/NetCoreConsoleServer
