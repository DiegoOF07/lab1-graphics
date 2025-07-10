mod framebuffer;
mod polygon;
mod line;

use framebuffer::Framebuffer;
use polygon::*;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);

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
    polygon_filled(&mut framebuffer, &polygon1, Color::YELLOW);
    polygon_outlined(&mut framebuffer, &polygon1, Color::WHITE);

    let polygon2 = vec![
        Vector2 { x: 321.0, y: 335.0 },
        Vector2 { x: 288.0, y: 286.0 },
        Vector2 { x: 339.0, y: 251.0 },
        Vector2 { x: 374.0, y: 302.0 },
    ];
    polygon_filled(&mut framebuffer, &polygon2, Color::BLUE);
    polygon_outlined(&mut framebuffer, &polygon2, Color::WHITE);

    let polygon3 = vec![
        Vector2 { x: 377.0, y: 249.0 },
        Vector2 { x: 411.0, y: 197.0 },
        Vector2 { x: 436.0, y: 249.0 }
    ];
    polygon_filled(&mut framebuffer, &polygon3, Color::RED);
    polygon_outlined(&mut framebuffer, &polygon3, Color::WHITE);

    let polygon4 = vec![
        Vector2 { x: 413.0, y: 177.0 },
        Vector2 { x: 448.0, y: 159.0 },
        Vector2 { x: 502.0, y: 88.0 },
        Vector2 { x: 553.0, y: 53.0 },
        Vector2 { x: 535.0, y: 36.0 },
        Vector2 { x: 676.0, y: 37.0 },
        Vector2 { x: 660.0, y: 52.0 },
        Vector2 { x: 750.0, y: 145.0 },
        Vector2 { x: 761.0, y: 179.0 },
        Vector2 { x: 672.0, y: 192.0 },
        Vector2 { x: 659.0, y: 214.0 },
        Vector2 { x: 615.0, y: 214.0 },
        Vector2 { x: 632.0, y: 230.0 },
        Vector2 { x: 580.0, y: 230.0 },
        Vector2 { x: 597.0, y: 215.0 },
        Vector2 { x: 552.0, y: 214.0 },
        Vector2 { x: 517.0, y: 144.0 },
        Vector2 { x: 466.0, y: 180.0 }
    ];
    polygon_filled(&mut framebuffer, &polygon4, Color::GREEN);
    polygon_outlined(&mut framebuffer, &polygon4, Color::WHITE);

    let polygon5 = vec![
        Vector2 { x: 682.0, y: 175.0 },
        Vector2 { x: 708.0, y: 120.0 },
        Vector2 { x: 735.0, y: 148.0 },
        Vector2 { x: 739.0, y: 170.0 }
    ];
    polygon_filled(&mut framebuffer, &polygon5, Color::BLACK);
    polygon_outlined(&mut framebuffer, &polygon5, Color::WHITE);

    let output_file_png = "out.png";
    let output_file_bmp = "out.bmp";

    framebuffer.render_to_file(output_file_png);
    framebuffer.render_to_file(output_file_bmp);
}
