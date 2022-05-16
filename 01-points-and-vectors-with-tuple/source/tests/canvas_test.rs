#[test]
fn test_canvas_property() {
    use source::canvas::*;

    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
}

#[test]
fn create_pixel_in_canvas() {
    use source::canvas::*;
    use source::color::Color;

    let mut c = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0); //red;
    c.write_pixel(1, 1, red);
    assert_eq!(c.pixel_at(0, 0), None);
}

#[test]
fn change_canvas_to_ppm_format() {
    use source::canvas::*;
    use source::color::Color;

    let mut c = Canvas::new(5, 3);
    let color1 = Color::new(1.5, 0.0, 0.0);
    let color2 = Color::new(0.0, 0.5, 0.0);
    let color3 = Color::new(-0.5, 0.0, 1.0);
    c.write_pixel(0, 0, color1);
    c.write_pixel(2, 1, color2);
    c.write_pixel(4, 2, color3);

    assert_eq!(c.canvas_to_ppm(), String::from("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"));
}

#[test]
fn canvas_to_ppm_format_with_70characters_limitation() {
    use source::canvas::*;
    use source::color::Color;

    let mut c = Canvas::new(10, 2);
    let color = Color::new(1.0, 0.8, 0.6);
    for i in 0..c.height + 1 {
        for j in 0..c.width + 1 {
            c.write_pixel(j, i, color);
        }
    }
    assert_eq!(c.canvas_to_ppm(), "P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n")
}
