
pub struct Disc {

}

impl Disc {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&self, pixels: &mut [u8], width: usize, height: usize) {
        let length = width as usize * height as usize * 4;
        for i in 0..length as usize {
            pixels[i] = 255;
        }
    }

}