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
    
    // Function to draw polygon 1 - yellow with white outline (5-pointed star)
    let draw_polygon_1 = |fb: &mut Framebuffer| {
        // Star coordinates provided by the teacher
        let polygon_1_vertices = vec![
            Vector2::new(165.0, 380.0),
            Vector2::new(185.0, 360.0),
            Vector2::new(180.0, 330.0),
            Vector2::new(207.0, 345.0),
            Vector2::new(233.0, 330.0),
            Vector2::new(230.0, 360.0),
            Vector2::new(250.0, 380.0),
            Vector2::new(220.0, 385.0),
            Vector2::new(205.0, 410.0),
            Vector2::new(193.0, 383.0),
        ];
        
        // Draw polygon 1 with yellow fill and white outline
        draw_polygon(fb, &polygon_1_vertices, Color::YELLOW, Color::WHITE);
    };
    
    draw_polygon_1(&mut framebuffer);
    framebuffer.render_to_file("out.bmp");

    while !window.window_should_close(){
        // Check if window was resized
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();
        
        if current_width != framebuffer.width() || current_height != framebuffer.height() {
            // Resize the framebuffer to match the new window size
            framebuffer.resize(current_width as u32, current_height as u32);
            framebuffer.clear();
            draw_polygon_1(&mut framebuffer);
        }
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
