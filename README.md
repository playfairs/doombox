# DOOMBOX

A bare-metal Rust project that boots directly into DOOM without any operating system.

```
=================     ===============     ===============   ========  ========
\\ . . . . . . .\\   //. . . . . . .\\   //. . . . . . .\\  \\. . .\\// . . //
||. . ._____. . .|| ||. . ._____. . .|| ||. . ._____. . .|| || . . .\/ . . .||
|| . .||   ||. . || || . .||   ||. . || || . .||   ||. . || ||. . . . . . . ||
||. . ||   || . .|| ||. . ||   || . .|| ||. . ||   || . .|| || . | . . . . .||
|| . .||   ||. _-|| ||-_ .||   ||. . || || . .||   ||. _-|| ||-_.|\ . . . . ||
||. . ||   ||-'  || ||  `-||   || . .|| ||. . ||   ||-'  || ||  `|\_ . .|. .||
|| . _||   ||    || ||    ||   ||_ . || || . _||   ||    || ||   |\ `-_/| . ||
||_-' ||  .|/    || ||    \|.  || `-_|| ||_-' ||  .|/    || ||   | \  / |-_.||
||    ||_-'      || ||      `-_||    || ||    ||_-'      || ||   | \  / |  `||
||    `'         || ||         `'    || ||    `'         || ||   | \  / |   ||
||            .===' `===.         .==='.`===.         .===' /==. |  \/  |   ||
||         .=='   \_|-_ `===. .==='   _|_   `===. .===' _-|/   `==  \/  |   ||
||      .=='    _-'    `-_  `='    _-'   `-_    `='  _-'   `-_  /|  \/  |   ||
||   .=='    _-'          `-__\._-'         `-_./__-'         `' |. /|  |   ||
||.=='    _-'                                                     `' |  /==.||
=='    _-'                                                            \/   `==
\   _-'                                                                `-_   /
 `''                                                                      ``'
```

>[!IMPORTANT]
> This Project is not complete and is not stable yet, so it is highly unrecommended to run this on bare-metal,
> if you want to test it on a Virtual Machine, or use QEMU, do that instead, and if you want to contribute,
> fork the Repository, and make a [**Pull Request**](https://github.com/playfairs/doombox/compare).
> ---
> Also, since I have never made something like this, it may take a while to create since I don't
> have the absolute patience needed to work on this actively. This will be updated when the project
> is in a stable state.


## Overview

**DOOMBOX** is a minimal kernel written in Rust that:
- Boots directly from BIOS/UEFI using GRUB
- Initializes x86_64 hardware (GDT, IDT, interrupts)
- Sets up VGA Mode 13h (320x200x256 colors)
- Runs a simplified DOOM-like engine with raycasting rendering

## Features

- **Bare-metal**: No OS dependency - runs directly on hardware
- **Rust-based**: Memory safe systems programming
- **Real-time rendering**: DOOM-like raycasting engine
- **Liveboot**: Creates bootable ISO for direct hardware boot
- **Minimal footprint**: Optimized for size and performance

## Building

### Prerequisites

**Using Nix (Recommended):**
```bash
nix develop
# or
direnv allow

./build.sh
```

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y grub-common grub-pc-bin grub2-common xorriso qemu-system-x86
```

**macOS:**
```bash
brew install qemu
```

**Rust target (if not using Nix):**
```bash
rustup target add x86_64-unknown-none
```

### Build Commands

```bash
# Using Nix (recommended)
nix develop
./build.sh

# Using Nix apps
nix run .#build    # Build ISO
nix run .#qemu     # Build and run in QEMU

# Traditional build
./build.sh

# Or use Makefile
make all          # Build everything
make iso          # Create only the ISO
make run          # Build and run in QEMU
make clean        # Clean build stuff
```


## Architecture

### Kernel Components

- **Bootloader**: GRUB multiboot2 compliant
- **Hardware Setup**: GDT, IDT, PIC, keyboard, timer
- **Memory Management**: Flat memory model, paging setup
- **Graphics**: VGA Mode 13h framebuffer (320x200x256)
- **Game Engine**: Raycasting renderer with floor/ceiling/walls

### DOOM Engine Features

- **Raycasting**: 3D-like perspective rendering
- **Animated walls**: Distance-based shading
- **Floor/ceiling**: Depth-based coloring
- **HUD**: Frame counter and DOOM text
- **Real-time**: Continuous rendering loop

## Running

### In QEMU
```bash
make run
./build.sh
```

### On Real Hardware
1. Build the ISO: `make iso`
2. Write to USB: `sudo dd if=doombox.iso of=/dev/sdX bs=4M`
3. Boot from USB drive

## Technical Details

### Memory Layout
- Kernel loaded at 1MB by GRUB
- VGA framebuffer at 0xA0000 (Mode 13h)
- Text mode buffer at 0xB8000
- Stack grows downward from kernel end

### Graphics Mode
- **Resolution**: 320x200 pixels
- **Colors**: 256-color palette
- **Memory**: Linear framebuffer at 0xA0000
- **Access**: Direct memory writes for pixel manipulation

### Interrupt Handling
- Timer: PIT channel 0 (IRQ0)
- Keyboard: PS/2 controller (IRQ1)
- PIC: 8259 Programmable Interrupt Controller
- Exceptions: Double fault, breakpoint, etc.

## Development

### Adding Features
- Extend `doom_engine.rs` for game logic
- Add new modules in `src/`
- Update `Cargo.toml` for dependencies
- Modify `linker.ld` for memory layout changes

### Debugging
- Serial output: `make run | tee serial.log`
- QEMU monitor: Press `Ctrl+A, C`
- GDB debugging: `make run-debug`

## Limitations

- Simplified DOOM engine (not full DOOM)
- No audio support
- Limited to VGA Mode 13h
- No network or disk I/O
- Basic input handling only

## Future Enhancements

- [ ] Full DOOM engine integration
- [ ] Audio support (PC Speaker/SoundBlaster)
- [ ] Higher resolution modes (VESA)
- [ ] Mouse support
- [ ] Save/load functionality
- [ ] Multiplayer support

## License

This project is for educational purposes. DOOM is copyrighted by id Software.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

---

**Note**: This is a bare-metal kernel project. It runs directly on hardware without any operating system. Use with caution and only on test systems or emulators.