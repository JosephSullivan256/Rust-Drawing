
pub struct Context {
    buffer: image::RgbaImage,
}

impl Context {
    pub fn new(buffer: image::RgbaImage) -> Context {
        Context { buffer }
    }

    // TODO remove randomize
    pub fn randomize(&mut self) {
        for (_x, _y, pixel) in self.buffer.enumerate_pixels_mut() {
            *pixel = image::Rgba([rand::random(),rand::random(),rand::random(),255]);
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel : image::Rgba<u8>) {
        self.buffer.put_pixel(x, y, pixel);
    }

    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x>0 && x < self.buffer.width() && y>0 && y < self.buffer.height()
    }

    pub fn get_buffer(self) -> image::RgbaImage {
        self.buffer
    }
}