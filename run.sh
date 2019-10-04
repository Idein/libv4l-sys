#!/bin/bash

set -ex

export TARGET_ARCH='arm-unknown-linux-gnueabihf'
export LIBCLANG_INCLUDE_PATH='/usr/include/clang/3.9.1/include'
export PKG_CONFIG_PATH='/raspbian/usr/lib/arm-linux-gnueabihf/pkgconfig'
export PKG_CONFIG_PREFIX='/raspbian/usr'

sudo apt update -y
sudo apt install -y gnupg apt-transport-https

sudo dpkg --add-architecture armhf
sudo apt update -y
sudo apt install -y libclang-dev:armhf

echo 'deb [arch=armhf] https://archive.raspbian.org/raspbian/ buster main contrib non-free rpi' | sudo tee /etc/apt/sources.list.d/raspbian.list
sudo wget -q -O- http://raspbian.raspberrypi.org/raspbian.public.key | sudo apt-key add -
sudo apt update -y

PKGLIST='libv4l-dev:armhf'
tempdir=`mktemp -d`
#sudo chown -R _apt.root ${tempdir}
sudo mkdir -p ${tempdir}
pushd ${tempdir}
  # download deb
  for pkg in ${PKGLIST}; do apt download ${pkg}; done
  
  # expand deb
  for pkg in *.deb; do sudo dpkg -x ${pkg} /raspbian; done
popd

cargo -v build --target=${TARGET_ARCH} \
&& cargo -v build --target=${TARGET_ARCH} --example v4l2grab

