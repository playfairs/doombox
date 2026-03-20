.PHONY: all clean build iso run

# Kernel Build
build:
	cargo build --release

kernel: build
	@mkdir -p build
	cp target/x86_64-unknown-none/release/doombox build/kernel.bin

iso: kernel
	@mkdir -p build/isofiles/boot/grub
	cp build/kernel.bin build/isofiles/boot/
	cp grub.cfg build/isofiles/boot/grub/
	grub-mkrescue -o doombox.iso build/isofiles

run: iso
	qemu-system-x86_64 -cdrom doombox.iso -m 512M -serial stdio

clean:
	cargo clean
	rm -rf build
	rm -f doombox.iso

install-deps:
	sudo apt-get update
	sudo apt-get install -y grub-common grub-pc-bin grub2-common xorriso nasm qemu-system-x86

install-deps-macos:
	brew install qemu

all: iso
