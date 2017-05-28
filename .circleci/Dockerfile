FROM ubuntu:17.04

RUN \
  apt-get update -qq && \
  apt-get install -y --no-install-recommends \
  build-essential \
  ca-certificates \
  curl \
  docker \
  libpq-dev \
  libsqlite3-dev \
  libssl-dev \
  pkg-config \
  sqlite3

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

ENV PATH=/root/.cargo/bin:$PATH
