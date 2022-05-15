#[test]
fn test_color_struct() {
    use source::color::Color;

    let c1 = Color::new(1.0, 0.0, 0.0);
    assert_eq!(c1.red, 1.0);
    assert_eq!(c1.green, 0.0);
    assert_eq!(c1.blue, 0.0);
}

#[test]
fn arithmetic_operation_of_two_colors() {
    use source::color::Color;
    use source::vector::Scalar;

    let c1 = Color::new(1.0, 2.0, 3.0);
    let c2 = Color::new(1.0, 4.0, 6.0);
    let s = Scalar { v: 2.0 };
    let s1 = Scalar { v: 2.0 };
    assert_eq!(c1 + c2, Color::new(2.0, 6.0, 9.0));
    assert_eq!(c2 - c1, Color::new(0.0, 2.0, 3.0));
    assert_eq!(c1 * s, Color::new(2.0, 4.0, 6.0));
    assert_eq!(c2 / s1, Color::new(0.5, 2.0, 3.0));
    assert_eq!(c2 / c1, Color::new(1.0, 2.0, 2.0));
    assert_eq!(c1 * c2, Color::new(1.0, 8.0, 18.0));
}
