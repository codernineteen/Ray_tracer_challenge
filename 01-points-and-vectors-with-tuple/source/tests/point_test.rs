#[test]
fn compare_points_with_default_set() {
    use source::point::*;

    let p0 = Point::new(4.3, -4.2, 3.1);
    assert_eq!(4.3, p0.x);
    assert_eq!(-4.2, p0.y);
    assert_eq!(3.1, p0.z);
    assert_eq!(1.0, p0.w);
}

#[test]
fn add_two_points() {
    use source::point::*;

    let p1 = Point::new(3.0, -2.0, 5.0);
    let p2 = Point::new(-2.0, 3.0, 1.0);
    assert_eq!(p1 + p2, Point::new(1.0, 1.0, 6.0));
}

#[test]
fn add_point_and_vector() {
    use source::point::*;
    use source::vector::*;

    let p3 = Point::new(3.0, -2.0, 5.0);
    let v3 = Vector::new(-2.0, 3.0, 1.0);
    assert_eq!(p3 + v3, Point::new(1.0, 1.0, 6.0));
}

#[test]
fn sub_point_from_point() {
    use source::point::Point;
    use source::vector::Vector;

    let p4 = Point::new(3.0, 2.0, 1.0);
    let p5 = Point::new(5.0, 6.0, 7.0);
    assert_eq!(p4 - p5, Vector::new(-2.0, -4.0, -6.0));
}

#[test]
fn sub_vector_from_point() {
    use source::point::Point;
    use source::vector::Vector;

    let p = Point::new(3.0, 2.0, 1.0);
    let v = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
}
