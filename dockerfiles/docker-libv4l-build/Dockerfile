FROM rust:1.36.0-buster

ENV LIBCLANG_INCLUDE_PATH '/usr/include/clang/7/include'

ARG RUST_CHANNEL
RUN test ${RUST_CHANNEL}

WORKDIR /tmp/libv4l_sys

RUN apt-get update -y \
    && apt-get install -y libv4l-dev libclang-7-dev

RUN rustup toolchain install ${RUST_CHANNEL}

COPY . /tmp/libv4l_sys

RUN rustup override set ${RUST_CHANNEL}
