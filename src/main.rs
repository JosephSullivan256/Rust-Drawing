use drawing::context::*;
use drawing::geometry::aabb::AABB;
use drawing::geometry::triangle::Triangle;
use drawing::geometry::polygon::Polygon;
use drawing::geometry::util::*;

fn main() {
    let buffer : image::RgbaImage = image::ImageBuffer::new(800, 800);

    let mut ctx = Context::new(buffer);
    //ctx.randomize();

    let rec = AABB::new(Vec2::new(100.,100.),Vec2::new(801.,801.));
    rec.fill(&mut ctx, image::Rgba([0,255,255,255]));

    let tri = Triangle::new(
        Vec2::new(0.0,0.0),
        Vec2::new(400.0,200.0),
        Vec2::new(200.0,500.0)
    );
    //tri.fill(&mut ctx, image::Rgba([0,0,255,255]));

    let poly = Polygon::new(vec![
        Vec2::new(0.0,0.0),
        Vec2::new(400.0,200.0),
        Vec2::new(200.0,500.0)
    ]);

    //let r = Ray { p: Vec2::new(50.,100.), d: Vec2::new(1.,0.)};
    //let s = Segment { a: Vec2::new(0.0,0.0), b: Vec2::new(400.0,200.0)};

    //println!("{}",intersect(&s,&r));
    poly.fill(&mut ctx, image::Rgba([0,0,255,255]));


    
    ctx.get_buffer().save("test.png").unwrap();
}

//let pixel = imgbuf.get_pixel_mut(x, y);
//let image::Rgb(data) = *pixel;
//*pixel = image::Rgb([data[0], i as u8, data[2]]);