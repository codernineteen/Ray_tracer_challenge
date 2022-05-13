#[test]
fn compare_composition_of_vector_with_default_set() {
    use source::vector::Vector;

    let v0 = Vector::new(4.3, -4.2, 3.1);
    assert_eq!(4.3, v0.x);
    assert_eq!(-4.2, v0.y);
    assert_eq!(3.1, v0.z);
    assert_eq!(0.0, v0.w);
}

#[test]
fn add_two_tuples() {
    use source::vector::Vector;

    let v1 = Vector::new(3.0, -2.0, 5.0);
    let v2 = Vector::new(-2.0, 3.0, 1.0);
    assert_eq!(v1 + v2, Vector::new(1.0, 1.0, 6.0));
}

#[test]
fn sub_each_vectors() {
    use source::vector::Vector;

    let v1 = Vector::new(3.0, 2.0, 1.0);
    let v2 = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
}

//how to create opposite directed vector(with zero vector subtraction)
#[test]
fn negate_a_vector() {
    use source::vector::Vector;

    let zero = Vector::new(0.0, 0.0, 0.0);
    let v1 = Vector::new(1.0, -1.0, 3.0);
    assert_eq!(zero - v1, Vector::new(-1.0, 1.0, -3.0));
}

#[test]
fn negate_vector_with_trait() {
    use source::vector::Vector;

    let v = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
}

#[test]
fn scalar_multiplication_of_vector() {
    use source::vector::*;

    let v = Vector::new(1.0, 2.0, 3.0);
    let s = Scalar { v: 2.0 };
    assert_eq!(v * s, Vector::new(2.0, 4.0, 6.0));
}

#[test]
fn scalar_division_of_vector() {
    use source::vector::*;

    let v = Vector::new(1.0, 2.0, 3.0);
    let s = Scalar { v: 5.0 };
    assert_eq!(v / s, Vector::new(0.2, 0.4, 0.6));
}

#[test]
fn magnitude_vector() {
    use source::vector::*;

    let v = Vector::new(2.0, 1.0, 0.0);
    let one_direction = Vector::new(0.0, 1.0, 0.0);
    assert_eq!(v.mag().round(), 2.0);
    assert_eq!(one_direction.mag(), 1.0)
}

#[test]
fn normalize_vector() {
    use source::vector::*;

    let v = Vector::new(4.0, 0.0, 0.0);
    assert_eq!(v.norm(), Vector::new(1.0, 0.0, 0.0));
}

#[test]
fn dot_product_two_vector() {
    use source::vector::*;

    let v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(4.0, 2.0, 1.0);
    assert_eq!(v1.dot(v2), 11.0);
}

#[test]
fn cross() {
    use source::vector::*;

    let v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
    assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
}
