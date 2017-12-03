FROM ubuntu:17.04

RUN \
  apt-get update -qq && \
  apt-get install -y --no-install-recommends \
  build-essential \
  ca-certificates \
  curl \
  docker \
  libpq-dev \
  libssl-dev \
  pkg-config \
  wget

ENV DOCKERIZE_VERSION v0.3.0
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
  tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
  rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

ENV PATH=/root/.cargo/bin:$PATH

RUN cargo install diesel_cli --no-default-features --features postgres
