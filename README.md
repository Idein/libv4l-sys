
# Libv4l-sys

[![CircleCI](https://circleci.com/gh/Idein/libv4l-sys.svg?style=svg)](https://circleci.com/gh/Idein/libv4l-sys)

Rust FFI wrapper to libv4l.

## Build

### Build by docker-compose

```sh
(host)$ docker-compose build libv4l-build
(host)$ docker-compose run --rm libv4l-build  # Just check if build succeeds

# Manually invoke build command inside container
(host)$ docker-compose run --rm libv4l-build bash
(container)$ cargo build
(container)$ file target/debug/liblibv4l_sys.rlib
target/debug/liblibv4l_sys.rlib: current ar archive
```

### Native build

You need to install the dependencies by yourself:

- libv4l (like `libv4l-dev`)
- libclang-7 (like `libclang-7-dev`)

And set the following environment variables:

- LIBCLANG_INCLUDE_PATH (e.g. `export LIBCLANG_INCLUDE_PATH=/usr/include/clang/7/include`)

Then execute:

```sh
cargo build
```

## Cross Build

### Into `arm-unknown-linux-gnueabihf`

```sh
(host)$ docker-compose build libv4l-build-cross-armhf
(host)$ docker-compose run --rm libv4l-build-cross-armhf  # Just check if build succeeds

# Manually invoke build command inside container
(host)$ docker-compose run --rm libv4l-build-cross-armhf bash
(container)$ cargo build --target=arm-unknown-linux-gnueabihf
(container)$ file target/arm-unknown-linux-gnueabihf/debug/liblibv4l_sys.rlib
target/arm-unknown-linux-gnueabihf/debug/liblibv4l_sys.rlib: current ar archive
```
