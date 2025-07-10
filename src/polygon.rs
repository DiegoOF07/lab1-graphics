use raylib::prelude::*;
use crate::line::line;
use crate::framebuffer::Framebuffer;

pub fn polygon_outlined(
    framebuffer: &mut Framebuffer,
    vertex: &[Vector2],
    color: Color
){
    if vertex.len() < 3 {
        return;
    }
    framebuffer.set_current_color(color);
    let size = vertex.len();
    for i in 0..(size-1){
        line(framebuffer, vertex[i], vertex[i+1]);
    }

    if vertex[0]!=vertex[size-1]{
        line(framebuffer, vertex[size-1], vertex[0]);
    }
}


pub fn polygon_filled(
    framebuffer: &mut Framebuffer, 
    vertex: &[Vector2], 
    color: Color
) {
    if vertex.len() < 3 {
        return;
    }

    framebuffer.set_current_color(color);

    let mut min_y = vertex[0].y as i32;
    let mut max_y = vertex[0].y as i32;
    for v in vertex {
        min_y = min_y.min(v.y as i32);
        max_y = max_y.max(v.y as i32);
    }

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..vertex.len() {
            let j = (i + 1) % vertex.len();
            let mut v1 = vertex[i];
            let mut v2 = vertex[j];

            if v1.y > v2.y {
                std::mem::swap(&mut v1, &mut v2);
            }

            if (y as f32) < v1.y || (y as f32) >= v2.y {
                continue;
            }

            let t = (y as f32 - v1.y) / (v2.y - v1.y);
            let x = v1.x + t * (v2.x - v1.x);
            intersections.push(x);
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 >= intersections.len() {
                break;
            }

            let x_start = intersections[i].ceil() as i32;
            let x_end = intersections[i + 1].floor() as i32;

            for x in x_start..=x_end {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}
