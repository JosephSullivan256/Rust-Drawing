
pub struct Context {
    buffer: image::RgbaImage,
}

impl Context {
    pub fn new(buffer: image::RgbaImage) -> Context {
        Context { buffer }
    }

    pub fn randomize(&mut self) {
        for (_x, _y, pixel) in self.buffer.enumerate_pixels_mut() {
            *pixel = image::Rgba([rand::random(),rand::random(),rand::random(),255]);
        }
    }

    pub fn get_buffer(self) -> image::RgbaImage {
        self.buffer
    }
}

// for x in 0..self.buffer.width() {
//     for y in 0..self.buffer.height() {
//         imgbuf.put_pixel(x, y, image::Rgb([rand::random(),rand::random(),rand::random()]));
//     }
// }