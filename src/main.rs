pub mod context;

fn main() {
    let buffer : image::RgbaImage = image::ImageBuffer::new(800, 800);

    let mut ctx = context::Context::new(buffer);
    ctx.randomize();
    
    ctx.get_buffer().save("test.png").unwrap();
}

//let pixel = imgbuf.get_pixel_mut(x, y);
//let image::Rgb(data) = *pixel;
//*pixel = image::Rgb([data[0], i as u8, data[2]]);