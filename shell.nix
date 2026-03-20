{ pkgs ? import <nixpkgs> { overlays = [ (import rust-overlay) ]; config.allowUnsupportedSystem = true; } }:

let
  rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
    targets = [ "x86_64-unknown-none" ];
    extensions = [ "rust-src" ];
  });
in

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustToolchain
    nasm
  ] ++ (if pkgs.system == "aarch64-darwin" || pkgs.system == "x86_64-darwin" then [
    qemu
  ] else [
    grub2
    xorriso
    qemu
  ]);

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  
  shellHook = ''
    echo "DOOMSHELL"
    echo "Rust toolchain: $(rustc --version)"
    echo "target: x86_64-unknown-none"
    echo ""
    echo "commands:"
    echo "  cargo build --release    # Build the kernel"
    echo "  ./build.sh               # Build and create ISO"
    echo "  make run                 # Build and run in QEMU"
    echo ""
    
    # Set up environment for nightly
    export RUSTC="${rustToolchain}/bin/rustc"
    export CARGO="${rustToolchain}/bin/cargo"
  '';
}
