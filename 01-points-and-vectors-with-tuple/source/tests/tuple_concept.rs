#[derive(Debug, PartialEq, Copy, Clone)]
struct PointOrVec {
    x: f64,
    y: f64,
    z: f64,
    is_vector: u8,
}

impl PointOrVec {
    fn new(x: f64, y: f64, z: f64, w: u8) -> Self {
        Self {
            x,
            y,
            z,
            is_vector: w,
        }
    }
}

#[test]
fn compare_each_points() {
    let p1 = PointOrVec::new(4.3, -4.2, 3.1, 0);
    assert_eq!(4.3, p1.x);
    assert_eq!(-4.2, p1.y);
    assert_eq!(3.1, p1.z);
    assert_eq!(0, p1.is_vector);
}

#[test]
fn cheack_vector_and_compare_its_composition() {
    let v1 = PointOrVec::new(4.3, -4.2, 3.1, 1);
    assert_eq!(4.3, v1.x);
    assert_eq!(-4.2, v1.y);
    assert_eq!(3.1, v1.z);
    assert_eq!(1, v1.is_vector);
}
