mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::draw_polygon;

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
    
    // Function to draw polygon 3 - red with white outline
    let draw_polygon_3 = |fb: &mut Framebuffer| {
        // Polygon 3 coordinates provided by the teacher
        let polygon_3_vertices = vec![
            Vector2::new(377.0, 249.0),
            Vector2::new(411.0, 197.0),
            Vector2::new(436.0, 249.0),
        ];
        
        // Draw polygon 3 with red fill and white outline
        draw_polygon(fb, &polygon_3_vertices, Color::RED, Color::WHITE);
    };
    
    draw_polygon_3(&mut framebuffer);
    framebuffer.render_to_file("out.bmp");
    while !window.window_should_close(){
        // Check if window was resized
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();
        
        if current_width != framebuffer.width() || current_height != framebuffer.height() {
            // Resize the framebuffer to match the new window size
            framebuffer.resize(current_width as u32, current_height as u32);
            framebuffer.clear();
            draw_polygon_3(&mut framebuffer);
        }
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
