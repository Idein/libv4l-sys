#!/bin/bash
set -ex

source env.sh

sudo apt update -y
sudo apt install -y --no-install-recommends libclang-3.9-dev

echo 'deb [arch=armhf] https://archive.raspbian.org/raspbian/ buster main contrib non-free rpi' | sudo tee /etc/apt/sources.list.d/raspbian.list
sudo wget -q -O- http://raspbian.raspberrypi.org/raspbian.public.key | sudo apt-key add -
sudo apt update -y

./fetch.sh

# PKGLIST='libv4l-dev:armhf libclang-common-7-dev:armhf'
PKGLIST='
  libv4l-dev_1.16.3-3_armhf.deb
  libclang-common-7-dev_1%3a7.0.1-8+rpi2_armhf.deb
  '

#SYSPKGLIST='linux-libc-dev:armhf libc6:armhf libc6-dev:armhf'
SYSPKGLIST='
  libc6-dev_2.28-10+rpi1_armhf.deb
  libc6_2.28-10+rpi1_armhf.deb
  linux-libc-dev_4.18.20-2+rpi1_armhf.deb
'

#PKGLIST=''
#SYSPKGLIST=''
tempdir=`mktemp -d`
mkdir -p ${tempdir}
cp *.deb ${tempdir}/
pushd ${tempdir}
  # # download deb
  # for pkg in ${PKGLIST}; do apt download ${pkg}; done
  
  # expand deb
  for pkg in ${PKGLIST}; do sudo dpkg -x ${pkg} /raspbian; done

  # # download deb
  # for pkg in ${SYSPKGLIST}; do apt download ${pkg}; done
  
  # expand deb
  for pkg in ${SYSPKGLIST}; do sudo dpkg -x ${pkg} /raspbian; done
  rm *.deb
popd

#sudo ln -s /raspbian/usr/include/arm-linux-gnueabihf /usr/include/
sudo ln -s /raspbian/lib/arm-linux-gnueabihf         /lib/
sudo ln -s /raspbian/usr/lib/arm-linux-gnueabihf     /usr/lib/

cargo -v build --target=${TARGET_ARCH} \
&& cargo -v build --target=${TARGET_ARCH} --example v4l2grab

