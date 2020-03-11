FROM rust:1.36.0-buster

ARG RUST_VER
RUN test ${RUST_VER}

WORKDIR /tmp/libv4l_sys

RUN apt-get update -y \
    && apt-get install -y libv4l-dev libclang-7-dev

RUN rustup install ${RUST_VER}

COPY . /tmp/libv4l_sys
