use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

pub type F = f32;

pub struct Disc {

    disc_x              : F,
    disc_y              : F,
    radius              : F,

    drag_start          : Option<(f32, f32)>,
    drag_disc_x         : F,
    drag_disc_y         : F,
}

impl Disc {

    pub fn new() -> Self {
        Self {
            disc_x      : 400.0,
            disc_y      : 300.0,
            radius      : 200.0,

            drag_start  : None,
            drag_disc_x : 0.0,
            drag_disc_y : 0.0,
        }
    }

    /// Draw a white circle accelerated via rayon for multithreading
    pub fn draw(&self, pixels: &mut [u8], width: usize, height: usize) {

        const LINES: usize = 1;
        let height = height as F;

        //let start = self.get_time();

        // Draw a 2D SDF Disc multithreaded via rayon

        pixels
            .par_rchunks_exact_mut(width * LINES * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width * LINES + i;

                    let x = (i % width) as F;
                    let y = height - (i / width) as F;

                    let mut color = [0.0, 0.0, 0.0, 1.0];

                    let dist = self.length((x - self.disc_x, y - self.disc_y)) - self.radius;

                    let mask = self.fill_mask(dist);
                    color = self.mix_color(&color, &[1.0, 1.0, 1.0, 1.0], mask);

                    pixel.copy_from_slice(&[(color[0] * 255.0) as u8, (color[1] * 255.0) as u8, (color[2] * 255.0) as u8, (color[3] * 255.0) as u8]);
                }
            });

        //let stop = self.get_time();
        //println!("tick time {:?}", stop - start);

    }

    /// Click / touch at the given position, check if we clicked inside the circle and if yes initialize dragging
    pub fn touch_down(&mut self, x: f32, y: f32) -> bool {
        let dist = self.length((x as F - self.disc_x, y as F - self.disc_y)) - self.radius;

        if dist <= 0.0 {
            // Clicked inside
            self.drag_start = Some((x, y));
            self.drag_disc_x = self.disc_x;
            self.drag_disc_y = self.disc_y;
        } else {
            self.drag_start = None;
        }
        false
    }

    pub fn touch_dragged(&mut self, x: f32, y: f32) -> bool {
        if let Some(drag_start) = self.drag_start {
            self.disc_x = self.drag_disc_x as F - (drag_start.0 as F - x as F);
            self.disc_y = self.drag_disc_y as F - (drag_start.1 as F - y as F);
            true
        } else {
            false
        }
    }

    pub fn touch_up(&mut self, _x: f32, _y: f32) -> bool {
        self.drag_start = None;
        false
    }

    /// Length of a 2d vector
    #[inline(always)]
    pub fn length(&self, v: (F, F)) -> F {
        ((v.0).powf(2.0) + (v.1).powf(2.0)).sqrt()
    }

    /// Fill mask for a 2D SDF shape
    #[inline(always)]
    pub fn fill_mask(&self, dist : F) -> F {
        (-dist).clamp(0.0, 1.0)
    }

    /// Mix two colors
    #[inline(always)]
    pub fn mix_color(&self, a: &[F], b: &[F], v: F) -> [F; 4] {
        [   (1.0 - v) * a[0] + b[0] * v,
            (1.0 - v) * a[1] + b[1] * v,

            (1.0 - v) * a[2] + b[2] * v,
            (1.0 - v) * a[3] + b[3] * v ]
    }

    /// Gets the current time in milliseconds
    fn _get_time(&self) -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let stop = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            stop.as_millis()
    }

}