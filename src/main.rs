mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title( "Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .resizable()  // Enable window resizing
        .build();
        

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::new(50,50,100,255));
    framebuffer.clear();
    
    // Function to draw the lines
    let draw_lines = |fb: &mut Framebuffer| {
        fb.set_current_color(Color::GREEN);
        line(fb, Vector2::new(100.0, 100.0), Vector2::new(fb.width() as f32 - 100.0, fb.height() as f32 - 100.0));

        fb.set_current_color(Color::RED);
        line(fb, Vector2::new(100.0, fb.height() as f32 - 100.0), Vector2::new(fb.width() as f32 - 100.0, 100.0));

        fb.set_current_color(Color::BLUE);
        line(fb, Vector2::new(200.0, fb.height() as f32 - 200.0), Vector2::new(fb.width() as f32 + 100.0, 200.0));
    };
    
    draw_lines(&mut framebuffer);

    while !window.window_should_close(){
        // Check if window was resized
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();
        
        if current_width != framebuffer.width() || current_height != framebuffer.height() {
            // Resize the framebuffer to match the new window size
            framebuffer.resize(current_width as u32, current_height as u32);
            framebuffer.clear();
            draw_lines(&mut framebuffer);
        }
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
