use drawing::context::*;
use drawing::geometry::aabb::AABB;
use drawing::geometry::util::*;

fn main() {
    let buffer : image::RgbaImage = image::ImageBuffer::new(800, 800);

    let mut ctx = Context::new(buffer);
    ctx.randomize();

    let rec = AABB::new(Vec2::new(1.,1.),Vec2::new(100.,100.));
    rec.draw(&mut ctx, image::Rgba([255,255,255,255]));
    
    ctx.get_buffer().save("test.png").unwrap();
}

//let pixel = imgbuf.get_pixel_mut(x, y);
//let image::Rgb(data) = *pixel;
//*pixel = image::Rgb([data[0], i as u8, data[2]]);