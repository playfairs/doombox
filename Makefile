.PHONY: all clean build iso run setup setup-rust setup-nix install-deps

# Full Setup
# For Nix specifically, if the Distribution is either NixOS, or the ~/nix or ~/.nix Directory is found, it uses the setup-nix command.
setup:
	@echo "DOOMBOX Setup"
	@echo "================"
	@echo "Do you want to setup with Nix? (y/N)"
	@read -r answer; \
	if [ "$$answer" != "y" ] && [ "$$answer" != "Y" ]; then \
		echo "Nix setup declined. Installing dependencies manually"; \
		$(MAKE) install-deps; \
		$(MAKE) setup-rust; \
	else \
		echo "Setting up with Nix..."; \
		$(MAKE) setup-nix; \
	fi

# Rust Setup for Bare Metal
setup-rust:
	@echo "Setting up Rust for bare-metal development"
	@if ! command -v rustup &> /dev/null; then \
		echo "Installing Rust..."; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		source ~/.cargo/env; \
	fi
	@echo "Installing nightly toolchain..."
	@rustup toolchain install nightly
	@rustup default nightly
	@echo "Adding bare-metal target..."
	@rustup target add x86_64-unknown-none
	@echo "Installing rust-src component..."
	@rustup component add rust-src --toolchain nightly
	@rustup component add rustfmt clippy
	@echo "Creating optimized Cargo config..."
	@mkdir -p .cargo
	@echo '[build]' > .cargo/config.toml
	@echo 'jobs = 1' >> .cargo/config.toml
	@echo '' >> .cargo/config.toml
	@echo '[target.x86_64-unknown-none]' >> .cargo/config.toml
	@echo 'rustflags = ["-C", "opt-level=0"]' >> .cargo/config.toml
	@echo '' >> .cargo/config.toml
	@echo '[unstable]' >> .cargo/config.toml
	@echo 'build-std = ["core", "compiler_builtins", "alloc"]' >> .cargo/config.toml
	@echo 'build-std-features = ["compiler-builtins-mem"]' >> .cargo/config.toml
	@echo "Rust setup complete!"

# Setup with Nix (recommended)
setup-nix:
	@echo "Setting up with Nix 󱄅"
	@if ! command -v nix &> /dev/null; then \
		echo "Installing Nix..."; \
		curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install; \
		echo "Please restart your terminal and run 'make setup' again"; \
		exit 0; \
	fi
	@echo "Nix setup complete! Use 'nix develop' to enter environment"

# Install distribution-specific dependencies
install-deps:
	@echo "Installing system dependencies."
	@if [ -f /etc/debian_version ] || [ -f /etc/ubuntu-release ]; then \
		echo "Detected Debian/Ubuntu-based system"; \
		sudo apt-get update; \
		sudo apt-get install -y build-essential grub-common grub-pc-bin grub2-common xorriso qemu-system-x86 pkg-config curl git; \
	elif [ -f /etc/fedora-release ] || [ -f /etc/redhat-release ]; then \
		echo "Detected Red Hat/Fedora-based system"; \
		sudo dnf install -y gcc grub2 xorriso qemu-system-x86 pkgconfig curl git make; \
	elif [ -f /etc/arch-release ]; then \
		echo "Detected Arch-based system"; \
		sudo pacman -S --noconfirm grub xorriso qemu pkgconf curl git base-devel; \
	elif [ -f /etc/os-release ]; then \
		. /etc/os-release; \
		if [ "$ID" = "opensuse-leap" ] || [ "$ID" = "opensuse-tumbleweed" ]; then \
			echo "Detected openSUSE-based system"; \
			sudo zypper install -y grub2 xorriso qemu-x86 pkg-config curl git make; \
		else \
			echo "Unsupported distribution: $$ID"; \
			echo "Please manually install: grub2, xorriso, qemu-system-x86, pkg-config, curl, git"; \
			exit 1; \
		fi; \
	else \
		echo "Cannot detect distribution"; \
		echo "Please manually install: grub2, xorriso, qemu-system-x86, pkg-config, curl, git"; \
		exit 1; \
	fi
	@echo "System dependencies installed!"

# Kernel Build
build:
	@echo "🔨 Building DOOMBOX kernel..."
	cargo build --release
	@echo "Build complete"

kernel: build
	@mkdir -p build
	cp target/x86_64-unknown-none/release/doombox build/kernel.bin
	@echo "Kernel binary created: build/kernel.bin"

iso: kernel
	@mkdir -p build/isofiles/boot/grub
	cp build/kernel.bin build/isofiles/boot/
	cp grub.cfg build/isofiles/boot/grub/
	@if command -v grub-mkrescue &> /dev/null; then \
		grub-mkrescue -o doombox.iso build/isofiles; \
		echo "ISO created: doombox.iso"; \
	else \
		echo "grub-mkrescue not found - cannot create ISO"; \
		echo "On macOS, use: qemu-system-x86_64 -kernel build/kernel.bin -m 512M -serial stdio"; \
	fi

run: iso
	@if [ -f doombox.iso ]; then \
		qemu-system-x86_64 -cdrom doombox.iso -m 512M -serial stdio; \
	else \
		echo "ISO not found, running kernel directly..."; \
		qemu-system-x86_64 -kernel build/kernel.bin -m 512M -serial stdio; \
	fi

run-kernel: kernel
	qemu-system-x86_64 -kernel build/kernel.bin -m 512M -serial stdio

clean:
	cargo clean
	rm -rf build
	rm -f doombox.iso
	@echo "Clean complete!"


# Development stuff
fmt:
	cargo fmt

clippy:
	cargo clippy

test-setup:
	@echo "🔍 Testing setup..."
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Target installed: $$(rustup target list --installed | grep x86_64-unknown-none || echo 'Not installed')"
	@if command -v nix &> /dev/null; then \
		echo "Nix version: $$(nix --version)"; \
	else \
		echo "Nix: Not installed"; \
	fi
	@if command -v grub-mkrescue &> /dev/null; then \
		echo "GRUB: Available"; \
	else \
		echo "GRUB: Not available"; \
	fi

help:
	@echo "DOOMBOX Makefile"
	@echo ""
	@echo ""
	@echo "Setup Commands:"
	@echo "  make setup           - Full setup (auto-detects system)"
	@echo "  make setup-rust      - Setup Rust toolchain only"
	@echo "  make setup-nix       - Setup with Nix (recommended)"
	@echo "  make install-deps    - Install system dependencies only"
	@echo ""
	@echo "Build Commands:"
	@echo "  make build           - Build kernel"
	@echo "  make kernel          - Build and copy kernel binary"
	@echo "  make iso             - Create bootable ISO"
	@echo "  make all             - Build everything (same as iso)"
	@echo ""
	@echo "Run Commands:"
	@echo "  make run             - Build and run in QEMU"
	@echo "  make run-kernel      - Run kernel directly (no ISO)"
	@echo ""
	@echo "Development:"
	@echo "  make fmt             - Format code"
	@echo "  make clippy          - Run linter"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make test-setup      - Verify installation"
	@echo "  make help            - Show this help"

all: iso
