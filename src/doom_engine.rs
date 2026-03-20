use core::f32::consts::PI;
use crate::println;

const VGA_WIDTH: usize = 320;
const VGA_HEIGHT: usize = 200;
const VGA_BUFFER: usize = 0xA0000;

struct GameState {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    frame_count: u32,
}

impl GameState {
    fn new() -> Self {
        Self {
            player_x: 0.0,
            player_y: 0.0,
            player_angle: 0.0,
            frame_count: 0,
        }
    }
    
    fn update(&mut self) {
        self.frame_count += 1;
        self.player_angle += 0.02;
        if self.player_angle >= 2.0 * PI {
            self.player_angle -= 2.0 * PI;
        }
    }
}

struct Renderer {
    buffer: *mut u8,
}

impl Renderer {
    fn new() -> Self {
        Self {
            buffer: VGA_BUFFER as *mut u8,
        }
    }
    
    fn clear_screen(&mut self) {
        unsafe {
            for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
                *self.buffer.add(i) = 0;
            }
        }
    }
    
    fn draw_floor(&mut self) {
        unsafe {
            for y in (VGA_HEIGHT / 2)..VGA_HEIGHT {
                for x in 0..VGA_WIDTH {
                    let offset = y * VGA_WIDTH + x;
                    let depth = (y - VGA_HEIGHT / 2) as f32 / (VGA_HEIGHT / 2) as f32;
                    let color = (depth * 64.0) as u8;
                    *self.buffer.add(offset) = color;
                }
            }
        }
    }
    
    fn draw_ceiling(&mut self) {
        unsafe {
            for y in 0..(VGA_HEIGHT / 2) {
                for x in 0..VGA_WIDTH {
                    let offset = y * VGA_WIDTH + x;
                    let depth = 1.0 - (y as f32 / (VGA_HEIGHT / 2) as f32);
                    let color = (depth * 32.0) as u8;
                    *self.buffer.add(offset) = color;
                }
            }
        }
    }
    
    fn draw_walls(&mut self, state: &GameState) {
        unsafe {
            for x in 0..VGA_WIDTH {
                let ray_angle = state.player_angle + (x as f32 - VGA_WIDTH as f32 / 2.0) * 0.005;
                
                let angle_normalized = ray_angle % (2.0 * PI);
                let sin_val = if angle_normalized < PI {
                    let t = angle_normalized / PI;
                    4.0 * t * (1.0 - t)
                } else {
                    let t = (angle_normalized - PI) / PI;
                    -4.0 * t * (1.0 - t)
                };
                
                let distance = 5.0 + sin_val.abs() * 3.0;
                let wall_height = (VGA_HEIGHT as f32 / distance) as usize;
                let wall_start = VGA_HEIGHT / 2 - wall_height / 2;
                let wall_end = VGA_HEIGHT / 2 + wall_height / 2;
                
                for y in wall_start..wall_end.min(VGA_HEIGHT) {
                    let offset = y * VGA_WIDTH + x;
                    let brightness = (255.0 / distance) as u8;
                    *self.buffer.add(offset) = brightness;
                }
            }
        }
    }
    
    fn draw_hud(&mut self, state: &GameState) {
        unsafe {
            let hud_text = [
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
            
            let start_y = VGA_HEIGHT - 120;
            let start_x = 5;
            
            for (row, line) in hud_text.iter().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    if ch != ' ' {
                        let y = start_y + row;
                        let x = start_x + col;
                        if y < VGA_HEIGHT && x < VGA_WIDTH {
                            let offset = y * VGA_WIDTH + x;
                            *self.buffer.add(offset) = 12;
                        }
                    }
                }
            }
            
            let frame_text = format_number(state.frame_count);
            let text_y = 10;
            let text_x = 10;
            
            for (i, ch) in frame_text.chars().enumerate() {
                if ch != ' ' {
                    let x = text_x + i * 8;
                    if x < VGA_WIDTH {
                        let offset = text_y * VGA_WIDTH + x;
                        *self.buffer.add(offset) = 15;
                    }
                }
            }
        }
    }
}

pub fn init() {
    println!("Initializing DOOM engine...");
 
    println!("DOOM engine initialized");
}

pub fn run() {
    println!("Running DOOM...");
    
    let mut state = GameState::new();
    let mut renderer = Renderer::new();
    
    loop {
        state.update();
        
        renderer.clear_screen();
        renderer.draw_floor();
        renderer.draw_ceiling();
        renderer.draw_walls(&state);
        renderer.draw_hud(&state);
        
        for _ in 0..100000 {
            x86_64::instructions::hlt();
        }
    }
}

fn format_number(num: u32) -> heapless::String<64> {
    let mut output = heapless::String::new();
    if num == 0 {
        let _ = output.push('0');
    } else {
        let mut n = num;
        let mut digits = [0u8; 10];
        let mut len = 0;
        while n > 0 {
            digits[len] = (n % 10) as u8 + b'0';
            n /= 10;
            len += 1;
        }
        for i in (0..len).rev() {
            let _ = output.push(digits[i] as char);
        }
    }
    output
}
