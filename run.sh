#!/bin/bash

set -ex

export TARGET_ARCH='arm-unknown-linux-gnueabihf'
## bindgen (running on host) requires libclang.so
export LIBCLANG_PATH='/usr/lib/llvm-3.9/lib'
## bindgen read header files for target arch
export LIBCLANG_INCLUDE_PATH='/raspbian/usr/include/clang/7.0.1/include'
export PKG_CONFIG_PATH='/raspbian/usr/lib/arm-linux-gnueabihf/pkgconfig'
export PKG_CONFIG_PREFIX='/raspbian/usr'

sudo apt update -y
sudo apt install -y --no-install-recommends gnupg apt-transport-https libclang-3.9-dev

echo 'deb [arch=armhf] https://archive.raspbian.org/raspbian/ buster main contrib non-free rpi' | sudo tee /etc/apt/sources.list.d/raspbian.list
sudo wget -q -O- http://raspbian.raspberrypi.org/raspbian.public.key | sudo apt-key add -
sudo apt update -y

PKGLIST='
  libv4l-dev:armhf
  libclang-common-7-dev:armhf
  '
SYSPKGLIST='
  linux-libc-dev:armhf
  libc6:armhf
  libc6-dev:armhf
  '
tempdir=`mktemp -d`
mkdir -p ${tempdir}
pushd ${tempdir}
  # download deb
  for pkg in ${PKGLIST}; do apt download ${pkg}; done
  
  # expand deb
  for pkg in *.deb; do sudo dpkg -x ${pkg} /raspbian; done
  rm *.deb

  # download deb
  for pkg in ${SYSPKGLIST}; do apt download ${pkg}; done
  
  # expand deb
  for pkg in *.deb; do sudo dpkg -x ${pkg} /; done
popd

cargo -v build --target=${TARGET_ARCH} \
&& cargo -v build --target=${TARGET_ARCH} --example v4l2grab

