use core::ptr;
use x86_64::VirtAddr;

const VGA_WIDTH: usize = 320;
const VGA_HEIGHT: usize = 200;
const VGA_BUFFER: usize = 0xA0000;

static mut DOOM_WAD: &[u8] = &[];

pub fn init() {
    println!("Initializing DOOM Engine...");

    unsafe {
        let vga_buffer = VGA_BUFFER as *mut u8;
        
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            *vga_buffer.add(i) = 0;
        }
    }
    
    println!("DOOM Engine initialized");
}

pub fn run() {
    println!("Running DOOM.");
    
    unsafe {
        let vga_buffer = VGA_BUFFER as *mut u8;
        
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let offset = y * VGA_WIDTH + x;
                let color = ((x as u16 * 255 / VGA_WIDTH as u16) + 
                            (y as u16 * 255 / VGA_HEIGHT as u16)) / 2;
                *vga_buffer.add(offset) = color as u8;
            }
        }
        
        let doom_pattern = [
                "=================     ===============     ===============   ========  ========",
                "\\ . . . . . . .\\   //. . . . . . .\\   //. . . . . . .\\  \\. . .\\// . . //",
                "||. . ._____. . .|| ||. . ._____. . .|| ||. . ._____. . .|| || . . .\\/ . . .||",
                "|| . .||   ||. . || || . .||   ||. . || || . .||   ||. . || ||. . . . . . . ||",
                "||. . ||   || . .|| ||. . ||   || . .|| ||. . ||   || . .|| || . | . . . . .||",
                "|| . .||   ||. _-|| ||-_ .||   ||. . || || . .||   ||. _-|| ||-_.|\\ . . . . ||",
                "||. . ||   ||-'  || ||  `-||   || . .|| ||. . ||   ||-'  || ||  `|\\_ . .|. .||",
                "|| . _||   ||    || ||    ||   ||_ . || || . _||   ||    || ||   |\\ `-_/| . ||",
                "||_-' ||  .|/    || ||    \\|.  || `-_|| ||_-' ||  .|/    || ||   | \\  / |-_.||",
                "||    ||_-'      || ||      `-_||    || ||    ||_-'      || ||   | \\  / |  `||",
                "||    `'         || ||         `'    || ||    `'         || ||   | \\  / |   ||",
                "||            .===' `===.         .==='.`===.         .===' /==. |  \\/  |   ||",
                "||         .=='   \\_|-_ `===. .==='   _|_   `===. .===' _-|/   `==  \\/  |   ||",
                "||      .=='    _-'    `-_  `='    _-'   `-_    `='  _-'   `-_  /|  \\/  |   ||",
                "||   .=='    _-'          `-__\\._-'         `-_./__-'         `' |. /|  |   ||",
                "||.=='    _-'                                                     `' |  /==.||",
                "=='    _-'                                                            \\/   `==",
                "\\   _-'                                                                `-_   /",
                " ''                                                                      ``'",
            ];
        
        let start_y = 50;
        let start_x = 80;
        
        for (row, line) in doom_pattern.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let y = start_y + row;
                    let x = start_x + col;
                    if y < VGA_HEIGHT && x < VGA_WIDTH {
                        let offset = y * VGA_WIDTH + x;
                        *vga_buffer.add(offset) = 12;
                    }
                }
            }
        }
    }
    
    println!("DOOM is running. Press ESC to quit.");
    
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn load_wad(wad_data: &[u8]) {
    unsafe {
        DOOM_WAD = wad_data;
    }
    println!("WAD file loaded: {} bytes", wad_data.len());
}
