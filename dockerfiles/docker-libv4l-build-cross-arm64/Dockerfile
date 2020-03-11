FROM rust:1.36.0-buster

ENV LIBCLANG_INCLUDE_PATH '/usr/include/clang/7/include'
ENV PKG_CONFIG_PATH '/usr/lib/aarch64-linux-gnu/pkgconfig'
ENV PKG_CONFIG_ALLOW_CROSS 1

ARG RUST_CHANNEL
RUN test ${RUST_CHANNEL}

WORKDIR /tmp/libv4l_sys

RUN dpkg --add-architecture arm64 \
    && apt-get update -y \
    && apt-get install -y libv4l-dev:arm64 libclang-common-7-dev:arm64 libclang-7-dev:arm64 \
    && apt-get install -y gcc-aarch64-linux-gnu libclang-7-dev

RUN rustup toolchain install ${RUST_CHANNEL} \
    && rustup target add --toolchain ${RUST_CHANNEL} aarch64-unknown-linux-gnu

COPY . /tmp/libv4l_sys

RUN rustup override set ${RUST_CHANNEL}
