mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::draw_polygon_with_hole;

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
    
    // Function to draw polygon 4 - green with white outline (with polygon 5 as hole)
    let draw_polygon_4 = |fb: &mut Framebuffer| {
        // Polygon 4 coordinates provided by the teacher
        let polygon_4_vertices = vec![
            Vector2::new(413.0, 177.0),
            Vector2::new(448.0, 159.0),
            Vector2::new(502.0, 88.0),
            Vector2::new(553.0, 53.0),
            Vector2::new(535.0, 36.0),
            Vector2::new(676.0, 37.0),
            Vector2::new(660.0, 52.0),
            Vector2::new(750.0, 145.0),
            Vector2::new(761.0, 179.0),
            Vector2::new(672.0, 192.0),
            Vector2::new(659.0, 214.0),
            Vector2::new(615.0, 214.0),
            Vector2::new(632.0, 230.0),
            Vector2::new(580.0, 230.0),
            Vector2::new(597.0, 215.0),
            Vector2::new(552.0, 214.0),
            Vector2::new(517.0, 144.0),
            Vector2::new(466.0, 180.0),
        ];
        
        // Polygon 5 coordinates (hole inside polygon 4)
        let polygon_5_vertices = vec![
            Vector2::new(682.0, 175.0),
            Vector2::new(708.0, 120.0),
            Vector2::new(735.0, 148.0),
            Vector2::new(739.0, 170.0),
        ];
        
        // Draw polygon 4 with polygon 5 as hole - green with white outline
        draw_polygon_with_hole(fb, &polygon_4_vertices, &polygon_5_vertices, Color::GREEN, Color::WHITE);
    };
    
    draw_polygon_4(&mut framebuffer);
    framebuffer.render_to_file("out.bmp");
    while !window.window_should_close(){
        // Check if window was resized
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();
        
        if current_width != framebuffer.width() || current_height != framebuffer.height() {
            // Resize the framebuffer to match the new window size
            framebuffer.resize(current_width as u32, current_height as u32);
            framebuffer.clear();
            draw_polygon_4(&mut framebuffer);
        }
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
