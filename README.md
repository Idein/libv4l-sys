
# Libv4l-sys

[![CircleCI](https://circleci.com/gh/Idein/libv4l-sys.svg?style=svg)](https://circleci.com/gh/Idein/libv4l-sys)

Rust FFI wrapper to libv4l.

## Build

```sh
cargo build
```

You can specify the path to the system header directory using `LIBCLANG_INCLUDE_PATH`.

```sh
LIBCLANG_INCLUDE_PATH=/usr/include/clang/7/include cargo build
```

### Cross build

```sh
libv4l-sys$ cat <<EOF > .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-rpi-linux-gnueabihf-gcc"
rustflags = ["-C", "link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf"]
EOF
libv4l-sys$ export LIBCLANG_INCLUDE_PATH=/usr/include/clang/7/include
libv4l-sys$ cargo build --target=arm-unknown-linux-gnueabihf
```

### Required package

- libclang-7-dev
- libv4l-dev
