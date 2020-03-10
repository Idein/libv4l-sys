FROM rust:1.36.0-buster

ENV LIBCLANG_INCLUDE_PATH '/usr/include/clang/7/include'
ENV PKG_CONFIG_PATH '/usr/lib/arm-linux-gnueabihf/pkgconfig'
ENV PKG_CONFIG_ALLOW_CROSS 1

ARG RUST_VER
RUN test ${RUST_VER}

ARG TARGET_ARCH
RUN test ${TARGET_ARCH}

WORKDIR /tmp/libv4l_sys

RUN dpkg --add-architecture armhf \
    && apt-get update -y \
    && apt-get install -y libv4l-dev:armhf libclang-common-7-dev:armhf libclang-7-dev:armhf \
    && apt-get install -y gcc-arm-linux-gnueabihf libclang-7-dev

RUN rustup install ${RUST_VER} \
    && rustup target add --toolchain ${RUST_VER} ${TARGET_ARCH}

COPY . /tmp/libv4l_sys
