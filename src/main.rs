mod bmp;
mod framebuffer;
mod patterns;
mod render;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};

const BLACK: u32 = 0x000000;

const FB_WIDTH: usize = 100;
const FB_HEIGHT: usize = 100;
const SCALE: usize = 6; // ventana = framebuffer * SCALE, para que se vea bien

fn main() {
    let mut framebuffer = Framebuffer::new(FB_WIDTH, FB_HEIGHT, BLACK);
    render::init(&mut framebuffer);

    let window_width = FB_WIDTH * SCALE;
    let window_height = FB_HEIGHT * SCALE;

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    // Limita el refresco para poder ver la animacion (equivale al "delay" entre frames)
    window.set_target_fps(10);

    let mut screen_buffer: Vec<u32> = vec![0; window_width * window_height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Escala el framebuffer (100x100) al tamano de la ventana
        for y in 0..FB_HEIGHT {
            for x in 0..FB_WIDTH {
                let color = framebuffer.get_color(x as i32, y as i32);
                for sy in 0..SCALE {
                    for sx in 0..SCALE {
                        let px = x * SCALE + sx;
                        let py = y * SCALE + sy;
                        screen_buffer[py * window_width + px] = color;
                    }
                }
            }
        }

        window
            .update_with_buffer(&screen_buffer, window_width, window_height)
            .unwrap();

        render::step(&mut framebuffer);
    }

    // Guarda el ultimo frame como imagen, por si lo quieres revisar
    framebuffer.render_to_file("out.bmp");
}