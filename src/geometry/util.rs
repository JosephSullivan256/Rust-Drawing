use nalgebra::Vector2;
use nalgebra::Matrix2;

pub type Vec2 = Vector2<f32>;

const EPSILON: f32 = 0.01;

pub struct Segment {
    pub a: Vec2,
    pub b: Vec2
}

pub struct Ray {
    pub p: Vec2,
    pub d: Vec2
}

pub fn intersect(s: &Segment, r: &Ray) -> bool {
    let A = Matrix2::from_columns(&[r.d,s.b-s.a]);
    let t = A.lu().solve(&(s.b-r.p)).unwrap();

    t[0]>=0.0+EPSILON && t[1]>=0.0 && t[1] <= 1.0
}