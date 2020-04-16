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

    pub fn iter(&self) -> impl Iterator<Item = (u32,u32)> {
        let x_iter = (self.p[0].floor() as u32)..(self.q[0].ceil() as u32);
        let y_iter = (self.p[1].floor() as u32)..(self.q[1].ceil() as u32);
        
        x_iter.flat_map(move |x| y_iter.clone().map(move |y| (x,y)))
    }

    pub fn fill(&self, ctx: &mut Context, color: image::Rgba<u8>) {
        for (x,y) in self.iter() {
            if ctx.in_bounds(x, y) {
                ctx.set_pixel(x, y, color);
            }
        }
    }
}