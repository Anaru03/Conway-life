use crate::framebuffer::Framebuffer;
use crate::patterns::{self, Pattern};

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;

pub fn init(framebuffer: &mut Framebuffer) {
    framebuffer.clear();

    let width = framebuffer.width as i32;
    let height = framebuffer.height as i32;

    let mut spawn = |pattern: Pattern, ox: i32, oy: i32| {
        for (x, y) in pattern {
            framebuffer.set_pixel(x + ox, y + oy, WHITE);
        }
    };

    // Generador pseudoaleatorio simple (LCG) para repartir organismos por todo el tablero
    let mut seed: u64 = 20260108;
    let mut next = move || -> u32 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (seed >> 33) as u32
    };

    type Gen = fn() -> Pattern;
    let generators: [Gen; 14] = [
        patterns::block,
        patterns::beehive,
        patterns::loaf,
        patterns::boat,
        patterns::tub,
        patterns::blinker,
        patterns::toad,
        patterns::beacon,
        patterns::pulsar,
        patterns::pentadecathlon,
        patterns::glider,
        patterns::glider,
        patterns::lwss,
        patterns::lwss,
    ];

    let cell = 9; // cuadricula de ~9px para repartir organismos por todo el tablero

    let mut gy = 0;
    while gy < height {
        let mut gx = 0;
        while gx < width {
            if next() % 100 < 55 {
                let idx = (next() as usize) % generators.len();
                let jitter_x = (next() % 3) as i32;
                let jitter_y = (next() % 3) as i32;
                spawn(generators[idx](), gx + jitter_x, gy + jitter_y);
            }
            gx += cell;
        }
        gy += cell;
    }
}

pub fn step(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width as i32;
    let height = framebuffer.height as i32;

    let previous = Framebuffer {
        width: framebuffer.width,
        height: framebuffer.height,
        buffer: framebuffer.buffer.clone(),
        background_color: framebuffer.background_color,
    };

    for y in 0..height {
        for x in 0..width {
            let alive = previous.get_color(x, y) == WHITE;
            let neighbors = count_neighbors(&previous, x, y);

            let next_alive = matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));

            framebuffer.set_pixel(x, y, if next_alive { WHITE } else { BLACK });
        }
    }
}

fn count_neighbors(framebuffer: &Framebuffer, x: i32, y: i32) -> u32 {
    let mut neighbors = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if framebuffer.get_color(x + dx, y + dy) == WHITE {
                neighbors += 1;
            }
        }
    }

    neighbors
}