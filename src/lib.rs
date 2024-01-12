mod utils;

use std::{fmt, mem, ops::{Index, IndexMut}, path::{Path, PathBuf}, fs::File, io::{self, Write}};

use utils::idx_to_coords;

/// Standard Pixel struct, contains r, g, b
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord { fn new(x: usize, y:usize) -> Self { Self { x, y } }}


impl Pixel {
    pub const BLACK: Self  = Self::new(0, 0, 0); 
    pub const WHITE: Self  = Self::new(255, 255, 255);
    pub const RED: Self    = Self::new(255, 0, 0); 
    pub const GREEN: Self  = Self::new(0, 255, 0); 
    pub const BLUE: Self   = Self::new(0, 0, 255); 
    pub const PURPLE: Self = Self::new(255, 0, 255);

    const fn new(r: u8, g: u8, b: u8) -> Self { 
        Self { r, g, b }
    }
}

trait PpmFormat {
    fn save_to_file(self, filepath: impl Into<PathBuf>) -> io::Result<()>;
}


struct ImagePPM {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl ImagePPM {
    fn new(width: usize, height: usize, bg_color: Pixel) -> Self {
        Self { width, height, pixels: vec![bg_color; width*height], }
    }
    fn get(&self, index: Coord) -> &Pixel {
        let i = index.x + (self.height - index.y - 1)*self.width;
        &self.pixels[i]
    }
    fn get_mut(&mut self, index: Coord) -> &mut Pixel {
        let i = index.x + (self.height - index.y - 1)*self.width;
        &mut self.pixels[i]
    }
}

impl PpmFormat for ImagePPM {
    fn save_to_file(self, filepath: impl Into<PathBuf>) -> Result<(), std::io::Error> {
        let mut file = File::create(filepath.into())?;
        file.write_all(format!("{}", self).as_bytes())?;
        Ok(())
    }
}

//impl Index<Coord> for ImagePPM {
//    type Output = Pixel;
//
//    fn index(&self, index: Coord) -> &Self::Output {
//        let i = index.x + (self.height - index.y - 1)*self.width;
//        &self.pixels[i]
//    }
//}
//
//impl IndexMut<Coord> for ImagePPM {
//    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
//        let i = index.x + (self.height - index.y - 1)*self.width;
//        &mut self.pixels[i]
//    }
//}

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

#[test]
fn bare_basics() {
    use crate::ImagePPM;

    let mut dot: ImagePPM = ImagePPM::new(3, 3, Pixel::PURPLE);
    dot[Coord::new(0, 0)] = Pixel::WHITE;
    println!("{dot}");
    dot[Coord::new(1, 0)] = Pixel::WHITE;
    println!("{dot}");
    dot[Coord::new(2, 0)] = Pixel::WHITE;
    println!("{dot}");

    dot[Coord::new(0, 1)] = Pixel::WHITE;
    println!("{dot}");
    dot[Coord::new(0, 2)] = Pixel::WHITE;
    println!("{dot}");


    dot[Coord::new(0, 1)] = Pixel::WHITE;
    println!("{dot}");
    dot[Coord::new(2, 1)] = Pixel::WHITE;
    println!("{dot}");

    dot[Coord::new(1, 1)] = Pixel::BLACK;
    println!("{dot}");

    dot[Coord::new(2, 2)] = Pixel::WHITE;
    dot[Coord::new(1, 2)] = Pixel::WHITE;
    println!("{dot}");


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
    let mut sq = ImagePPM::new(255, 255, Pixel::BLACK);
    for (i, pixel) in sq.pixels.iter_mut().enumerate() {
        let Coord { x, y } = idx_to_coords(i, sq.width);
        pixel.r = x as u8;
        pixel.g = y as u8;
    }

    sq.save_to_file("test_outputs/TEST_color_wheel.ppm").unwrap();

}
