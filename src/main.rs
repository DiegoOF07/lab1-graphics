mod framebuffer;
mod polygon;
mod line;

use framebuffer::Framebuffer;
use polygon::polygon;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::BLUE);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    framebuffer.set_current_color(Color::BLUE);
    let polygon1 = vec![
        Vector2 { x: 165.0, y: 380.0 },
        Vector2 { x: 185.0, y: 360.0 },
        Vector2 { x: 180.0, y: 330.0 },
        Vector2 { x: 207.0, y: 345.0 },
        Vector2 { x: 233.0, y: 330.0 },
        Vector2 { x: 230.0, y: 360.0 },
        Vector2 { x: 250.0, y: 380.0 },
        Vector2 { x: 220.0, y: 385.0 },
        Vector2 { x: 205.0, y: 410.0 },
        Vector2 { x: 193.0, y: 383.0 },
    ];
    polygon(&mut framebuffer, &polygon1);

    framebuffer.set_current_color(Color::RED);
    let polygon2 = vec![
        Vector2 { x: 321.0, y: 335.0 },
        Vector2 { x: 288.0, y: 286.0 },
        Vector2 { x: 339.0, y: 251.0 },
        Vector2 { x: 374.0, y: 302.0 },
    ];
    polygon(&mut framebuffer, &polygon2);

    let output_file = "polygons.png";

    framebuffer.render_to_file(output_file);
}
