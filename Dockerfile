FROM ubuntu:latest

ADD . /TRACKER

RUN apt-get update && \
    apt-get install -y gcc g++ make cmake  \
    build-essential curl

#Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo --version

WORKDIR /TRACKER

RUN cargo build

EXPOSE 3333

ENTRYPOINT ["cargo", "run"]