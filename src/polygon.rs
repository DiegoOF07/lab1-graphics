use raylib::prelude::*;
use crate::line::line;
use crate::framebuffer::Framebuffer;

pub fn polygon(
    framebuffer: &mut Framebuffer,
    points: &[Vector2]
){
    let size = points.len();
    for i in 0..(size-1){
        line(framebuffer, points[i], points[i+1]);
    }

    if points[0]!=points[size-1]{
        line(framebuffer, points[size-1], points[0]);
    }
}