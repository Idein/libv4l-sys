export TARGET_ARCH='arm-unknown-linux-gnueabihf'
## bindgen read header files for target arch
export LIBCLANG_INCLUDE_PATH='/raspbian/usr/include/clang/7.0.1/include'
export PKG_CONFIG_PATH='/raspbian/usr/lib/arm-linux-gnueabihf/pkgconfig'
export PKG_CONFIG_SYSROOT_DIR='/raspbian'
export PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1
