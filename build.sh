#!/bin/bash

set -e

echo "Building DOOMBOX."

mkdir -p build/isofiles/boot/grub

echo "Building kernel."
cargo build --release

# Copies the Kernel to the ISO
echo "Preparing ISO files..."
cp target/x86_64-unknown-none/release/doombox build/isofiles/boot/kernel.bin 2>/dev/null || \
cp target/x86_64-doombox/release/doombox build/isofiles/boot/kernel.bin 2>/dev/null || {
    echo "Error: Could not find built kernel"
    echo "Available targets:"
    find target -name "doombox" -type f 2>/dev/null || echo "No doombox binary found"
    exit 1
}

cp grub.cfg build/isofiles/boot/grub/

echo "Making ISO, please wait and do not cancel."
if command -v grub-mkrescue >/dev/null 2>&1; then
    grub-mkrescue -o doombox.iso build/isofiles
elif command -v grub2-mkrescue >/dev/null 2>&1; then
    grub2-mkrescue -o doombox.iso build/isofiles
else
    echo "Error: grub-mkrescue not found. Please install GRUB tools."
    exit 1
fi

echo "Build complete. doombox.iso created."

if command -v qemu-system-x86_64 >/dev/null 2>&1; then
    echo "Run in QEMU? (y/n)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        echo "Starting QEMU..."
        qemu-system-x86_64 -cdrom doombox.iso -m 512M -serial stdio
    fi
else
    echo "QEMU not found. Install qemu-system-x86_64 to test the ISO."
fi
