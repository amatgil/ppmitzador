use ppmitzador::{utils::idx_to_coords, Coord, ImagePBM, ImagePPM, Pixel, PpmFormat};

#[test]
fn color_square() {

    let mut sq = ImagePPM::new(255, 255, Pixel::BLACK);
    let w = sq.width();
    for (i, pixel) in sq.atoms_mut().iter_mut().enumerate() {
        let Coord { x, y } = idx_to_coords(i, w);
        pixel.r = x as u8;
        pixel.g = y as u8;
    }

    sq.save_to_file("test_outputs/TEST_color_wheel.ppm").unwrap();

}
#[test]
fn bw_square() {
    let mut sq = ImagePBM::new(255, 255, false);
    sq.draw_circle(Coord { x: 100, y: 100 }, 30, true);

    sq.save_to_file("test_outputs/TEST_bw_square.pbm").unwrap();

}
