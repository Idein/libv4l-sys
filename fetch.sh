#!/bin/bash
set -uxe

PKGLIST='libv4l-dev:armhf libclang-common-7-dev:armhf'
SYSPKGLIST='linux-libc-dev:armhf libc6:armhf libc6-dev:armhf'

for pkg in ${PKGLIST} ${SYSPKGLIST}
do
	if [ -e "${pkg}" ]; then
		echo "${pkg} is already exists"
	else
		apt download ${pkg}
	fi
done

