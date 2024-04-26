FROM ubuntu:jammy-20230301
#RUN apt-get update &&\
#    apt-get install -y heaptrack &&\
#    apt-get install -y heaptrack-gui
# RUN apt-get update &&\
#    apt -y install pkg-config libfreetype6-dev libfontconfig1-dev mingw-w64
COPY kmpi-rust /tmp/kmpi-rust
RUN chmod go+r /tmp/kmpi-rust
WORKDIR "/tmp"
ENTRYPOINT ["/tmp/kmpi-rust"]
#ENTRYPOINT ["tail", "-f", "/dev/null"]

