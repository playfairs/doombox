{
  description = "DOOMSHELL - Nix Shell Environment for DOOMBOX";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnsupportedSystem = true;
        };

        rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          targets = [ "x86_64-unknown-none" ];
          extensions = [ "rust-src" ];
        });

        buildInputs = with pkgs; [
          rustToolchain
          nasm
        ] ++ (if system == "aarch64-darwin" || system == "x86_64-darwin" then [
          qemu
        ] else [
          grub2
          xorriso
          qemu
        ]);

        nativeBuildInputs = with pkgs; [
          pkg-config
        ] ++ (if system == "aarch64-darwin" || system == "x86_64-darwin" then [
        ] else [
        ]);
      in
      {
        devShell = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          
          # Set up nightly toolchain as default
          shellHook = ''
            echo "DOOMSHELL"
            echo "Rust toolchain: $(rustc --version)"
            echo "target: x86_64-unknown-none"
            echo ""
            echo "commands:"
            echo "  cargo build --release    # Builds the kernel"
            echo "  ./build.sh               # Build and create ISO"
            echo "  make run                 # Build and run in QEMU"
            echo ""
            
            # Set up environment for nightly
            export RUSTC="${rustToolchain}/bin/rustc"
            export CARGO="${rustToolchain}/bin/cargo"
          '';
        };

        packages.default = pkgs.stdenv.mkDerivation {
          name = "doombox";
          src = ./.;

          inherit buildInputs nativeBuildInputs;

          buildPhase = ''
            export RUST_TARGET_PATH=$(pwd)
            cargo build --release --target x86_64-unknown-none
            mkdir -p build/isofiles/boot/grub
            cp target/x86_64-unknown-none/release/doombox build/isofiles/boot/kernel.bin
            cp grub.cfg build/isofiles/boot/grub/
            ${if (pkgs.system == "aarch64-darwin" || pkgs.system == "x86_64-darwin") then ''
              echo "Warning: Cannot create ISO on macOS due to GRUB unavailability"
              echo "Please run this on Linux or use the build script directly"
              exit 1
            '' else ''
              grub-mkrescue -o doombox.iso build/isofiles
            ''}
          '';

          installPhase = ''
            mkdir -p $out
            ${if (pkgs.system == "aarch64-darwin" || pkgs.system == "x86_64-darwin") then ''
              echo "Installing kernel binary only on macOS"
              cp target/x86_64-unknown-none/release/doombox $out/
            '' else ''
              cp doombox.iso $out/
              cp target/x86_64-unknown-none/release/doombox $out/
            ''}
          '';

          meta = with pkgs.lib; {
            description = "Bare-metal Rust project that boots directly into DOOM";
            license = licenses.mit;
            platforms = platforms.linux ++ platforms.darwin;
          };
        };

        apps.qemu = {
          type = "app";
          program = "${pkgs.writeShellScript "run-doombox" ''
            if [ ! -f doombox.iso ]; then
              echo "Building DOOMBOX..."
              ${if (pkgs.system == "aarch64-darwin" || pkgs.system == "x86_64-darwin") then ''
                echo "Cannot build ISO on macOS. Please run on Linux system."
                echo "You can still build the kernel with: cargo build --release --target x86_64-unknown-none"
                exit 1
              '' else ''
                nix build .#default
                cp result/doombox.iso ./
              ''}
            fi
            echo "Starting QEMU."
            qemu-system-x86_64 -cdrom doombox.iso -m 512M -serial stdio
          ''}";
        };

        apps.build = {
          type = "app";
          program = "${pkgs.writeShellScript "build-doombox" ''
            ${if (pkgs.system == "aarch64-darwin" || pkgs.system == "x86_64-darwin") then ''
              echo "Building kernel only on macOS (ISO creation requires Linux)"
              cargo build --release --target x86_64-unknown-none
              echo "Kernel built: target/x86_64-unknown-none/release/doombox"
            '' else ''
              nix build .#default
              cp result/doombox.iso ./
              echo "DOOMBOX ISO created: doombox.iso"
            ''}
          ''}";
        };
      });
}
