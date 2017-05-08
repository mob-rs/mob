FROM ubuntu:17.04

RUN \
  apt-get update -qq && \
  apt-get install -y --no-install-recommends \
	ca-certificates \
  curl \
  sqlite3

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

ENV PATH=/root/.cargo/bin:$PATH
