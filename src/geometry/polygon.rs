use super::util::*;
use super::aabb::AABB;
use crate::context::Context;

pub struct Polygon{
    verts: Vec<Vec2>
}

impl Polygon {
    pub fn new(verts: Vec<Vec2>) -> Polygon{
        Polygon { verts }
    }

    pub fn bound(&self) -> AABB {
        let mut inf = self.verts[0];
        let mut sup = self.verts[0];
        for v in &self.verts {
            inf = nalgebra::inf(&v, &inf);
            sup = nalgebra::sup(&v, &sup);
        }
        AABB::new(inf, sup)
    }

    pub fn inside(&self, x: Vec2) -> bool {
        let r = Ray {p: x, d: Vec2::new(1.0,0.01)}; // slightly upwards to mitigate issues with corners being skipped/double counted
        let mut count = 0;

        for (v1,v2) in self.verts.iter().zip(self.verts.iter().chain(&[self.verts[0]]).skip(1)){
            let s = Segment {a: *v1, b: *v2};
            if intersect(&s, &r) {
                count += 1;
            }
        }
        count % 2 == 1
    }

    pub fn fill(&self, ctx: &mut Context, color: image::Rgba<u8>) {
        for (x,y) in self.bound().iter() {
            if ctx.in_bounds(x, y) && self.inside(Vec2::new(x as f32,y as f32)){
                ctx.set_pixel(x, y, color);
            }
        }
    }
}