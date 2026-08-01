mod bmp;
mod framebuffer;
mod patterns;
mod render;

use framebuffer::Framebuffer;

const BLACK: u32 = 0x000000;

fn main() {
    let mut framebuffer = Framebuffer::new(100, 100, BLACK);

    render::render(&mut framebuffer);

    framebuffer.render_to_file("out.bmp");
}