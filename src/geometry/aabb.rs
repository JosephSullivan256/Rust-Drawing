use crate::context::Context;
use super::util::*;

pub struct AABB {
    p: Vec2, // top left corner
    q: Vec2, // bottom right corner
}

impl AABB {
    pub fn new(p: Vec2, q: Vec2) -> AABB {
        AABB {p, q}
    }

    pub fn draw(&self, ctx: &mut Context, color: image::Rgba<u8>) {
        for x in (self.p[0].floor() as u32)..(self.q[0].floor() as u32){
            for y in (self.p[1].floor() as u32)..(self.q[1].floor() as u32){
                ctx.set_pixel(x, y, color);
            }
        }
    }
}