use crate::framebuffer::Framebuffer;

const WHITE: u32 = 0xFFFFFF;

pub fn render(framebuffer: &mut Framebuffer) {
    framebuffer.clear();

    framebuffer.set_pixel(10, 10, WHITE);
    framebuffer.set_pixel(11, 10, WHITE);
    framebuffer.set_pixel(12, 10, WHITE);

    framebuffer.set_pixel(10, 11, WHITE);
    framebuffer.set_pixel(12, 11, WHITE);

    framebuffer.set_pixel(10, 12, WHITE);
    framebuffer.set_pixel(11, 12, WHITE);
    framebuffer.set_pixel(12, 12, WHITE);
}