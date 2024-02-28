use ppmitzador::{utils::idx_to_coords, Coord, ImagePBM, ImagePPM, Pixel, PpmFormat};

#[test]
fn bare_basics() {

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
