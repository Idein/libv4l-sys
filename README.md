
# Libv4l-sys

[![CircleCI](https://circleci.com/gh/Idein/libv4l-sys.svg?style=svg)](https://circleci.com/gh/Idein/libv4l-sys)

Rust FFI wrapper to libv4l.

## Build

```sh
cargo build
```

### Parameter

You can specify some build parameters.

- `LIBCLANG_INCLUDE_PATH`: Path to the system header directory

    ```sh
    LIBCLANG_INCLUDE_PATH=/usr/include/clang/7/include cargo build
    ```

- `TARGET_ARCH`: Target architecture

    ```sh
    TARGET_ARCH=arm-unknown-linux-gnueabihf cargo build
    ```

### Cross build

For cross compiling, some more configurations are required.

#### Example (build for armhf)

```sh
libv4l-sys$ cat <<EOF > .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-rpi-linux-gnueabihf-gcc"
rustflags = ["-C", "link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf"]
EOF
libv4l-sys$ export LIBCLANG_INCLUDE_PATH=/usr/include/clang/7/include
libv4l-sys$ export TARGET_ARCH=arm-unknown-linux-gnueabihf
libv4l-sys$ cargo build --target=${TARGET_ARCH}
```

### Required package

- libclang-7-dev
- libv4l-dev
