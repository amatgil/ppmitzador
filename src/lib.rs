mod utils;

use std::{fmt, fs::File, io::{self, Write}, ops::{self, Add, Sub}, path::PathBuf};

/// Basic RGB Pixel struct
#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}

impl Coord {
    pub fn new(x: usize, y:usize) -> Self { Self { x, y } }
    pub fn abs(&self) -> f64 { ((self.x*self.x + self.y*self.y) as f64).sqrt() }
    pub fn distance(&self, rhs: Self) -> f64 {
        let ax = self.x as isize;
        let ay = self.y as isize;
        let bx = rhs.x as isize;
        let by = rhs.y as isize;

        let dx = (ax-bx).abs() as usize;
        let dy = (ay-by).abs() as usize;
        Coord::new(dx, dy).abs()
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self { x: self.x + rhs.x, y: self.y + rhs.y, } }
}
impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self { x: self.x - rhs.x, y: self.y - rhs.y, } }
}

impl Pixel {
    pub const BLACK: Self  = Self::new(0, 0, 0); 
    pub const UNIT: Self  = Self::new(1, 1, 1); 
    pub const WHITE: Self  = Self::new(255, 255, 255);
    pub const RED: Self    = Self::new(255, 0, 0); 
    pub const GREEN: Self  = Self::new(0, 255, 0); 
    pub const BLUE: Self   = Self::new(0, 0, 255); 
    pub const PURPLE: Self = Self::new(255, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self { 
        Self { r, g, b }
    }
}

pub trait PpmFormat {
    fn save_to_file(self, filepath: impl Into<PathBuf>) -> io::Result<()>;
}

/// Basic image file type
pub struct ImagePPM {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl ImagePPM {
    pub fn new(width: usize, height: usize, bg_color: Pixel) -> Self {
        Self { width, height, pixels: vec![bg_color; width*height], }
    }
    /// Get value of pixel at coordinates (bottom left is (0, 0)). None value means it was OOB
    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x >= self.width || y >= self.height { return None; }
        let i = x + (self.height - y - 1)*self.width;
        Some(&self.pixels[i])
    }
    /// Get mutable access to pixel at coordinates (bottom left is (0, 0)). None value means it was OOB
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x >= self.width || y >= self.height { return None; }
        let i = x + (self.height - y - 1)*self.width;
        Some(&mut self.pixels[i])
    }
    pub fn draw_line(&mut self, a: Coord, b: Coord, col: Pixel) {
        let (ax, ay, bx, by) = (a.x as f64, a.y as f64, b.x as f64, b.y as f64);
        let dist = ((ax-bx)*(ax-bx) + (ay-by)*(ay-by)).sqrt();
        let mut t = 0.0;
        while t <= dist {
            let x = ax + (bx - ax)*(t/dist);
            let y = ay + (by - ay)*(t/dist);
            *self.get_mut(x as usize, y as usize).unwrap() = col;
            t += 1.0;
        }

        *self.get_mut(b.x, b.y).unwrap() = col;
    }
    pub fn draw_line_with_thickness(&mut self, a: Coord, b: Coord, col: Pixel, thickness: usize) {
        let (ax, ay, bx, by) = (a.x as f64, a.y as f64, b.x as f64, b.y as f64);
        let dist = ((ax-bx)*(ax-bx) + (ay-by)*(ay-by)).sqrt();
        let mut t = 0.0;
        while t <= dist {
            let x = ax + (bx - ax)*(t/dist);
            let y = ay + (by - ay)*(t/dist);
            //*self.get_mut(x as usize, y as usize).unwrap() = col;
            self.draw_circle(Coord { x: x as usize, y: y as usize }, thickness, col);
            t += 1.0;
        }

        *self.get_mut(b.x, b.y).unwrap() = col;
    }
    /// Draw a circle (taxicab distance metric). Assumes that it will fit, will likely panic if it
    /// doesn't
    pub fn draw_circle(&mut self, center: Coord, radius: usize, col: Pixel) {
        let r = radius as isize / 2;
        for dx in -r..r {
            for dy in -r..r {
                let c = Coord {
                    x: (center.x as isize+ dx).max(0) as usize,
                    y: (center.y as isize + dy).max(0) as usize,
                };
                *self.get_mut(c.x, c.y).unwrap() = col;
            }
        }
    }
}

pub struct ImagePBM {
    /// False for background (black), true for foreground (white)
    pixels: Vec<bool>,
    width: usize,
    height: usize,
}


impl ImagePBM {
    pub fn new(width: usize, height: usize) -> Self { Self { width, height, pixels: vec![false; width*height], } }
    /// Get value of pixel at coordinates (bottom left is (0, 0)). None value means it was OOB
    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width || y >= self.height { return None; }
        let i = x + (self.height - y - 1)*self.width;
        Some(self.pixels[i])
    }
    /// Get mutable access to pixel at coordinates (bottom left is (0, 0)). None value means it was OOB
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        if x >= self.width || y >= self.height { return None; }
        let i = x + (self.height - y - 1)*self.width;
        Some(&mut self.pixels[i])
    }
    pub fn draw_line(&mut self, a: Coord, b: Coord, col: bool) {
        let (ax, ay, bx, by) = (a.x as f64, a.y as f64, b.x as f64, b.y as f64);
        let dist = ((ax-bx)*(ax-bx) + (ay-by)*(ay-by)).sqrt();
        let mut t = 0.0;
        while t <= dist {
            let x = ax + (bx - ax)*(t/dist);
            let y = ay + (by - ay)*(t/dist);
            *self.get_mut(x as usize, y as usize).unwrap() = col;
            t += 1.0;
        }

        *self.get_mut(b.x, b.y).unwrap() = col;
    }
    pub fn draw_line_with_thickness(&mut self, a: Coord, b: Coord, col: bool, thickness: usize) {
        let (ax, ay, bx, by) = (a.x as f64, a.y as f64, b.x as f64, b.y as f64);
        let dist = ((ax-bx)*(ax-bx) + (ay-by)*(ay-by)).sqrt();
        let mut t = 0.0;
        while t <= dist {
            let x = ax + (bx - ax)*(t/dist);
            let y = ay + (by - ay)*(t/dist);
            self.draw_circle(Coord { x: x as usize, y: y as usize }, thickness, col);
            t += 1.0;
        }

        *self.get_mut(b.x, b.y).unwrap() = col;
    }
    /// Draw a circle (taxicab distance metric). Assumes that it will fit, will likely panic if it
    /// doesn't
    pub fn draw_circle(&mut self, center: Coord, radius: usize, col: bool) {
        let r = radius as isize / 2;
        for dx in -r..r {
            for dy in -r..r {
                let c = Coord {
                    x: (center.x as isize+ dx).max(0) as usize,
                    y: (center.y as isize + dy).max(0) as usize,
                };
                *self.get_mut(c.x, c.y).unwrap() = col;
            }
        }
    }
}

impl PpmFormat for ImagePPM {
    fn save_to_file(self, filepath: impl Into<PathBuf>) -> Result<(), std::io::Error> {
        let mut file = File::create(filepath.into())?;
        file.write_all(format!("{}", self).as_bytes())?;
        Ok(())
    }
}

impl fmt::Display for ImagePPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const BYTES_PER_PIXEL: usize = 3 * 3 + 2;

        let mut out = String::with_capacity(self.width * self.height * BYTES_PER_PIXEL);
        out.push_str("P3\n");
        out.push_str(&format!("{} {}\n", self.width, self.height));
        out.push_str("255\n");

        for pixel in &self.pixels {
            out.push_str(&format!("{:3} {:3} {:3}\n", pixel.r, pixel.g, pixel.b));
        }

        write!(f, "{}", out)
    }
}

impl fmt::Display for ImagePBM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(self.width * self.height);
        out.push_str("P1\n");
        out.push_str(&format!("{} {}\n", self.width, self.height));

        for pixel in &self.pixels { out.push_str(&format!("{}", *pixel as usize)); }

        write!(f, "{}", out)
    }
}

impl ops::Mul<u8> for Pixel {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            r : self.r * rhs,
            g : self.g * rhs,
            b : self.b * rhs,
        }
    }
}

impl PpmFormat for ImagePBM {
    fn save_to_file(self, filepath: impl Into<PathBuf>) -> io::Result<()> {
        let mut file = File::create(filepath.into())?;
        file.write_all(format!("{}", self).as_bytes())?;
        Ok(())
    }
}

#[test]
fn bare_basics() {
    use crate::ImagePPM;

    let mut dot: ImagePPM = ImagePPM::new(3, 3, Pixel::PURPLE);
    *dot.get_mut(0, 0).unwrap() = Pixel::WHITE;
    *dot.get_mut(1, 0).unwrap() = Pixel::WHITE;
    *dot.get_mut(2, 0).unwrap() = Pixel::WHITE;

    *dot.get_mut(0, 1).unwrap() = Pixel::WHITE;
    *dot.get_mut(0, 2).unwrap() = Pixel::WHITE;

    *dot.get_mut(0, 1).unwrap() = Pixel::WHITE;
    *dot.get_mut(2, 1).unwrap() = Pixel::WHITE;

    *dot.get_mut(1, 1).unwrap() = Pixel::BLACK;

    *dot.get_mut(2, 2).unwrap() = Pixel::WHITE;
    *dot.get_mut(1, 2).unwrap() = Pixel::WHITE;

    println!("{dot}");

    let expected = 
r#"P3
3 3
255
255 255 255
255 255 255
255 255 255
255 255 255
  0   0   0
255 255 255
255 255 255
255 255 255
255 255 255
"#;
    assert_eq!(expected, format!("{dot}"));
}

#[test]
fn color_square() {
    use utils::idx_to_coords;

    let mut sq = ImagePPM::new(255, 255, Pixel::BLACK);
    for (i, pixel) in sq.pixels.iter_mut().enumerate() {
        let Coord { x, y } = idx_to_coords(i, sq.width);
        pixel.r = x as u8;
        pixel.g = y as u8;
    }

    sq.save_to_file("test_outputs/TEST_color_wheel.ppm").unwrap();

}
#[test]
fn bw_square() {
    let mut sq = ImagePBM::new(255, 255);
    sq.draw_circle(Coord { x: 100, y: 100 }, 30, true);

    sq.save_to_file("test_outputs/TEST_bw_square.pbm").unwrap();

}
