#!/bin/bash

# Stop the script if any command fails
set -e

# Clean previous builds
cargo clean

# Build the project using the nightly target
cargo build 

# Convert the ELF file to a binary file (kernel image)
aarch64-elf-objcopy -O binary target/aarch64-unknown-none/debug/rusty_pi kernel8.img

# Check if the boot volume is mounted
if [ ! -d "/Volumes/BOOT" ]; then
  echo "Boot volume not found. Ensure the SD card is mounted as BOOT."
  exit 1
fi

# Copy kernel8.img and config.txt to the mounted volume
cp kernel8.img config.txt /Volumes/BOOT/

# Unmount the disk
diskutil unmountDisk /dev/disk6

echo "Completed"