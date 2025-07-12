use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

/// Fill a polygon using a better scanline algorithm for concave polygons
pub fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vector2]) {
    if vertices.len() < 3 {
        return; // Need at least 3 vertices for a polygon
    }

    // Find bounding box
    let mut min_y = vertices[0].y as i32;
    let mut max_y = vertices[0].y as i32;
    let mut min_x = vertices[0].x as i32;
    let mut max_x = vertices[0].x as i32;
    
    for vertex in vertices {
        let y = vertex.y as i32;
        let x = vertex.x as i32;
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
    }

    // Use point-in-polygon test for each pixel
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if point_in_polygon(x as f32, y as f32, vertices) {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}

/// Test if a point is inside a polygon using the ray casting algorithm
fn point_in_polygon(x: f32, y: f32, vertices: &[Vector2]) -> bool {
    let mut inside = false;
    let n = vertices.len();
    
    let mut j = n - 1;
    for i in 0..n {
        let xi = vertices[i].x;
        let yi = vertices[i].y;
        let xj = vertices[j].x;
        let yj = vertices[j].y;
        
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    
    inside
}

/// Draw polygon outline
pub fn draw_polygon_outline(framebuffer: &mut Framebuffer, vertices: &[Vector2]) {
    if vertices.len() < 3 {
        return;
    }

    // Draw lines between consecutive vertices
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = vertices[(i + 1) % vertices.len()];
        line(framebuffer, start, end);
    }
}

/// Draw a filled polygon with outline
pub fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vector2], fill_color: Color, outline_color: Color) {
    // Draw fill first
    framebuffer.set_current_color(fill_color);
    fill_polygon(framebuffer, vertices);
    
    // Draw outline on top
    framebuffer.set_current_color(outline_color);
    draw_polygon_outline(framebuffer, vertices);
}
