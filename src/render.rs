use crate::framebuffer::Framebuffer;
use crate::patterns::{self, Pattern};

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;

/// Dibuja el patron inicial: varios organismos clasicos repartidos por la pantalla.
pub fn init(framebuffer: &mut Framebuffer) {
    framebuffer.clear();

    let mut spawn = |pattern: Pattern, ox: i32, oy: i32| {
        for (x, y) in pattern {
            framebuffer.set_pixel(x + ox, y + oy, WHITE);
        }
    };

    // Still lifes
    spawn(patterns::block(), 5, 5);
    spawn(patterns::beehive(), 15, 5);
    spawn(patterns::loaf(), 25, 5);
    spawn(patterns::boat(), 35, 5);
    spawn(patterns::tub(), 45, 5);

    // Oscillators
    spawn(patterns::blinker(), 10, 20);
    spawn(patterns::toad(), 20, 20);
    spawn(patterns::beacon(), 30, 20);
    spawn(patterns::pulsar(), 55, 15);
    spawn(patterns::pentadecathlon(), 5, 45);

    // Spaceships (gliders y LWSS, con espacio libre delante para que "vuelen")
    spawn(patterns::glider(), 70, 5);
    spawn(patterns::glider(), 5, 70);
    spawn(patterns::glider(), 40, 40);
    spawn(patterns::lwss(), 60, 70);
    spawn(patterns::lwss(), 20, 85);
}

/// Calcula la siguiente generacion aplicando las 4 reglas de Conway.
/// Usa unicamente get_color (via una foto/snapshot de la generacion actual)
/// y set_pixel para escribir el resultado, tal como pide el enunciado.
pub fn step(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width as i32;
    let height = framebuffer.height as i32;

    // Snapshot de la generacion actual para no leer valores ya actualizados.
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