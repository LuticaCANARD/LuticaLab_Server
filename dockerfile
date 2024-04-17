FROM ubuntu
FROM ubuntu:20.04
RUN apt-get -qq update \
&& apt-get -qq install -y g++ cmake wget unzip \
&& apt-get clean

EXPOSE 8000
ENTRYPOINT ["/bin/bash"]