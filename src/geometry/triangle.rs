use super::util::*;
use super::aabb::AABB;
use crate::context::Context;

pub struct Triangle {
    a: Vec2,
    b: Vec2,
    c: Vec2
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle {
        Triangle{ a, b, c}
    }

    pub fn inside(&self, x: Vec2) -> bool {
        let xab = (x-self.a).perp(&(self.b-self.a));
        let xbc = (x-self.b).perp(&(self.c-self.b));
        let xca = (x-self.c).perp(&(self.a-self.c));
        
        (xab>=0.0 && xbc>=0.0 && xca>=0.0) || (xab<=0.0 && xbc<=0.0 && xca<=0.0)
    }

    pub fn bound(&self) -> AABB {
        let inf = nalgebra::inf(&nalgebra::inf(&self.a, &self.b), &self.c);
        let sup = nalgebra::sup(&nalgebra::sup(&self.a, &self.b), &self.c);
        AABB::new(inf, sup)
    }

    pub fn fill(&self, ctx: &mut Context, color: image::Rgba<u8>) {
        for (x,y) in self.bound().iter() {
            if ctx.in_bounds(x, y) && self.inside(Vec2::new(x as f32,y as f32)){
                ctx.set_pixel(x, y, color);
            }
        }
    }
}