pub mod utils;
use std::{fmt::{self, Display}, fs::File, io::Write, ops::{self, Add, Sub}, path::PathBuf};

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
        let dx = (self.x as isize - rhs.x as isize).abs() as usize;
        let dy = (self.y as isize - rhs.y as isize).abs() as usize;
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

impl ops::Mul<u8> for Pixel {
    type Output = Self;
    fn mul(self, rhs: u8) -> Self::Output { Self { r : self.r * rhs, g : self.g * rhs, b : self.b * rhs, } }
}


impl Pixel {
    pub const BLACK: Self  = Self::new(0, 0, 0); 
    pub const UNIT: Self  = Self::new(1, 1, 1); 
    pub const WHITE: Self  = Self::new(255, 255, 255);
    pub const RED: Self    = Self::new(255, 0, 0); 
    pub const GREEN: Self  = Self::new(0, 255, 0); 
    pub const BLUE: Self   = Self::new(0, 0, 255); 
    pub const PURPLE: Self = Self::new(255, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
}

pub trait PpmFormat: Display {
    type Atom: Copy;

    // Minimum implementation
    fn new(width: usize, height: usize, bg_color: Self::Atom) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn atoms(&self) -> &Vec<Self::Atom>;
    fn atoms_mut(&mut self) -> &mut Vec<Self::Atom>;

    // Default implementations
    /// Get value of pixel at coordinates (bottom left is (0, 0)). None value means it was OOB
    fn get(&self, x: usize, y: usize) -> Option<&Self::Atom> {
        if x >= self.width() || y >= self.height() { return None; }

        Some(&self.atoms()[x + (self.height() - y - 1)*self.width()])
    }

    /// Get mutable access to pixel at coordinates (bottom left is (0, 0)). None value means it was OOB
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Atom> {
        if x >= self.width() || y >= self.height() { return None; }
        let i = x + (self.height() - y - 1)*self.width();
        Some(&mut self.atoms_mut()[i])
    }

    fn draw_circle(&mut self, center: Coord, radius: usize, col: Self::Atom) {
        let r = radius as isize / 2;
        for dx in -r..r {
        for dy in -r..r {
            *self.get_mut(
                (center.x as isize + dx).max(0) as usize,
                (center.y as isize + dy).max(0) as usize
            ).unwrap() = col;
        }
        }
    }

    /// Written by Gerard, uses the parametric equation to fill pixels
    fn draw_line(&mut self, a: Coord, b: Coord, col: Self::Atom) {
        let (ax, ay, bx, by) = (a.x as f64, a.y as f64, b.x as f64, b.y as f64);
        let dist = ((ax-bx)*(ax-bx) + (ay-by)*(ay-by)).sqrt();
        let mut t = 0.0;
        while t <= dist {
            let x = ax + (bx - ax)*(t / dist);
            let y = ay + (by - ay)*(t / dist);
            *self.get_mut(x as usize, y as usize).unwrap() = col;
            t += 1.0;
        }

        *self.get_mut(b.x, b.y).unwrap() = col;
    }

    /// Adapting Gerard's, uses the parametric equation to fill in circles instead of pixels
    fn draw_line_with_thickness(&mut self, a: Coord, b: Coord, col: Self::Atom, thickness: usize) {
        let (ax, ay, bx, by) = (a.x as f64, a.y as f64, b.x as f64, b.y as f64);
        let dist = ((ax-bx)*(ax-bx) + (ay-by)*(ay-by)).sqrt();
        let mut t = 0.0;
        while t <= dist {
            let x = ax + (bx - ax)*(t / dist);
            let y = ay + (by - ay)*(t / dist);
            self.draw_circle(Coord { x: x as usize, y: y as usize }, thickness, col);
            t += 1.0;
        }

        *self.get_mut(b.x, b.y).unwrap() = col;
    }

    /// Draw a circle (taxicab distance metric). Assumes that it will fit, will likely panic if it
    /// doesn't
    fn save_to_file(&self, filepath: impl Into<PathBuf>) -> Result<(), std::io::Error> {
        let mut file = File::create(filepath.into())?;
        file.write_all(format!("{}", self).as_bytes())?;
        Ok(())
    }
}

/// Basic image file type
pub struct ImagePPM {
    atoms: Vec<Pixel>,
    width: usize,
    height: usize,
}

pub struct ImagePBM {
    /// False for background (black), true for foreground (white)
    atoms: Vec<bool>,
    width: usize,
    height: usize,
}

impl PpmFormat for ImagePPM {
    type Atom = Pixel;

    fn new(width: usize, height: usize, bg_color: Pixel) -> Self { Self { width, height, atoms: vec![bg_color; width*height], } }
    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
    fn atoms(&self) -> &Vec<Pixel> { &self.atoms }
    fn atoms_mut(&mut self) -> &mut Vec<Pixel> { &mut self.atoms }

}

impl PpmFormat for ImagePBM {
    type Atom = bool;

    fn new(width: usize, height: usize, val: bool) -> Self { Self { width, height, atoms: vec![val; width*height], } }
    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
    fn atoms(&self) -> &Vec<bool> { &self.atoms }
    fn atoms_mut(&mut self) -> &mut Vec<Self::Atom> { &mut self.atoms }
}


impl fmt::Display for ImagePPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(self.width * self.height * (3 * 4) + 40);
        out.push_str(&format!("P3\n{} {}\n255\n", self.width, self.height));

        for pixel in &self.atoms { out.push_str(&format!("{:3} {:3} {:3}\n", pixel.r, pixel.g, pixel.b)); }

        write!(f, "{}", out)
    }
}

impl fmt::Display for ImagePBM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(self.width * self.height + 20);
        out.push_str(&format!("P1\n{} {}\n", self.width, self.height));

        for pixel in &self.atoms { out.push_str(&format!("{}", !pixel as usize)); }
        out.push('\n');

        write!(f, "{}", out)
    }
}
