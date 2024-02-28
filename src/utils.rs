use crate::Coord;

pub fn coords_to_idx(c: Coord, w: usize) -> usize { c.x + w*c.y }

pub fn idx_to_coords(i: usize, w: usize) -> Coord {
    Coord {
        x: i % w,
        y: i / w, 
    }
}
