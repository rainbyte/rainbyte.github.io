---
title: Restore files on RPi homelab server
author: rainbyte
published: 2026-03-08 03:06:00
tags: debian, kernel, linux, rpi
language: en
---

I lost connection to my Raspberry Pi homelab server and then I found it wouldn't
respond to SSH. After connecting a debug screen, I discovered a message about
corrupted `libapparmor` library.

Since the RPi uses aarch64 architecture, I wasn't able to easily chroot from my
x86 PC, so I had to restore affected files directly from `.deb` packages to the
RPi microsd card.

This guide explains how to recover a corrupted Debian system on Raspberry Pi by
manually extracting and replacing system files. So let's go!

## Summary

1. RPi server stopped responding
2. Screen showed `libapparmor` corruption error
3. Cannot chroot because RPi is aarch64, PC is on x86
4. Restored broken library from `.deb` directly to microsd
5. Booted again, but got kernel panic
6. Restored kernel modules from `.deb` to microsd
7. Booted successfully
8. Installed `debsums` to verify packages integrity
9. Reinstalled other broken packages via apt
10. System fully restored

<!-- more -->

## Solve the first error

These are the step-by-step instructions:

1. First, identify the microsd card partition and mount it:

   ```sh
   mkdir /tmp/rootfs
   sudo mount /dev/sdXN /tmp/rootfs
   ```

   Notes:
   - Find the correct device with `lsblk` or `blkid`
   - Replace `sdXN` with that partition (eg. `sdb2`)

2. Identify the corrupted file by checking error messages from the boot screen,
   which in my case mentioned about issues with `libapparmor` library

3. Get the right `.deb` package according to Debian system version:

   ```sh
   curl -o libapparmor1_4.1.0-1_arm64.deb \
     http://http.us.debian.org/debian/pool/main/a/apparmor/libapparmor1_4.1.0-1_arm64.deb
   ```

   Note: check `/etc/os-release` on your system for the correct version

4. Extract `data.tar.xz` from package contents:

   ```sh
   ar x libapparmor1_4.1.0-1_arm64.deb
   ```

   Note: it can be `data.tar.gz` for older packages

5. Extract data contents

   - For modern `.xz` compressed archives:

     ```sh
     tar -xJf data.tar.xz
     ```

   - For older `.gz` compressed archives:

     ```sh
     tar -xzf data.tar.gz
     ```

6. Verify if the library is corrupted by comparing the extracted file with the
   one on the mounted microsd card partition:

   ```sh
   cmp -s \
     usr/lib/aarch64-linux-gnu/libapparmor.so.1.24.2 \
     /tmp/rootfs/lib/aarch64-linux-gnu/libapparmor.so.1.24.2 \
     && echo "MATCH" || echo "MISMATCH"
   ```

   Note: it says `MATCH` if files are identical, or `MISMATCH` if they differ
7. Replace the library if there is a mismatch

   ```sh
   cp \
     usr/lib/aarch64-linux-gnu/libapparmor.so.1.24.2 \
     /tmp/rootfs/lib/aarch64-linux-gnu/libapparmor.so.1.24.2
   ```

8. Run the `cmp` command again to confirm the files now match
9. Fix permissions if needed

   ```sh
   sudo chmod 644 /tmp/rootfs/lib/aarch64-linux-gnu/libapparmor.so.1.24.2
   ```

## Kernel recovery

After fixing the library, I booted the microsd card but got a kernel panic,
which indicated kernel modules were also corrupted.

So these steps were the next to be applied:

1. Download and extract kernel package

   ```sh
   curl -o linux-image-6.12.62+rpt-rpi-v8_6.12.62-1+rpt1_arm64.deb https://archive.raspberrypi.com/debian/pool/main/l/linux/linux-image-6.12.62+rpt-rpi-v8_6.12.62-1+rpt1_arm64.deb
   ar x linux-image-6.12.62+rpt-rpi-v8_6.12.62-1+rpt1_arm64.deb
   tar -xJf data.tar.xz
   ```

   Note: Use the kernel version matching your system
2. Compare boot files between the extracted package and mounted microsd fat32
   boot partition:

   ```sh
   cmp -s \
     boot/vmlinuz-6.12.62+rpt-rpi-v8 \
     /tmp/rootfs/boot/vmlinuz-6.12.62+rpt-rpi-v8 \
     && echo "MATCH" || echo "MISMATCH"

   cmp -s \
     boot/config-6.12.62+rpt-rpi-v8 \
     /tmp/rootfs/boot/config-6.12.62+rpt-rpi-v8 \
     && echo "MATCH" || echo "MISMATCH"

   cmp -s \
     boot/System.map-6.12.62+rpt-rpi-v8 \
     /tmp/rootfs/boot/System.map-6.12.62+rpt-rpi-v8 \
     && echo "MATCH" || echo "MISMATCH"
   ```

3. Generate checksums for modules from the extracted package:

   ```sh
   cd usr/lib/modules/6.12.62+rpt-rpi-v8/
   find . -type f -exec sha256sum {} \; | sort > /tmp/hashes_orig.txt
   ```

4. Generate checksums for modules on the microsd card:

   ```sh
   cd /tmp/rootfs/usr/lib/modules/6.12.62+rpt-rpi-v8/
   find . -type f -exec sha256sum {} \; | sort > /tmp/hashes_card.txt
   ```

   Note: being inside is needed to have same output
5. Compare the two checksums lists:

   ```sh
   diff -u /tmp/hashes_orig.txt /tmp/hashes_card.txt
   ```

6. If there are differences then replace affected modules by copying the entire
   modules directory or at least overriding the broken files:

   ```sh
   sudo cp -r \
     usr/lib/modules/6.12.62+rpt-rpi-v8/* \
     /tmp/rootfs/usr/lib/modules/6.12.62+rpt-rpi-v8/
   ```

## Verify and repair packages

Once the system booted, in order to verify package integrity I got some tools:

1. Install debsums

   ```sh
   sudo apt update
   sudo apt install debsums
   ```

2. Run a silent check to list only corrupted files:

   ```sh
   sudo debsums -s
   ```

3. For each corrupted package do a reinstall, example:

   ```sh
   sudo apt install --reinstall \
     libacl1 libassuan9 libcamera0.6 libcc1-0 libcrypt-dev \
     libxmuu1 rpi-usb-gadget vim-common
   ```

   Note: replace with the packages shown by `debsums`
4. Verify integrity again

   ```sh
   sudo debsums -s
   ```

   Note: if no output, then all packages are intact

## Conclusion

The system is now fully restored!

This process saved me from re-imaging the microsd card and losing all
configuration.

It seems I must use an UPS next time >.<

`Happy hacking!` 🐱
