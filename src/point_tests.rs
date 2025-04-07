use crate::finite_fields::FieldElement;
use crate::point::{G, Point, SECP256K1_N};
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::string::ToString;

// Helper function to create a Point from hexadecimal coordinates
fn point_from_hex(x_hex: &str, y_hex: &str) -> Point {
    let x_bigint = BigInt::parse_bytes(x_hex.as_bytes(), 16).unwrap();
    let y_bigint = BigInt::parse_bytes(y_hex.as_bytes(), 16).unwrap();
    let x = FieldElement::new(x_bigint).unwrap();
    let y = FieldElement::new(y_bigint).unwrap();
    Point::new(Some(x), Some(y)).unwrap()
}

//---------------------
// Point Creation Tests
//---------------------

#[test]
fn test_valid_point() {
    // Test that a valid point (generator G) is created successfully
    let p = Point::new(Some(G.x().clone()), Some(G.y().clone()));
    assert!(p.is_ok(), "Generator point should satisfy y^2 = x^3 + 7");
    assert_eq!(p.unwrap(), *G);
}

#[test]
fn test_invalid_point() {
    // Test that an invalid point (1, 2) is rejected
    let x = FieldElement::new(BigInt::from(1)).unwrap();
    let y = FieldElement::new(BigInt::from(2)).unwrap();
    let p = Point::new(Some(x), Some(y));
    assert!(p.is_err(), "Point (1, 2) should be invalid");
    let error_msg = p.unwrap_err();
    assert!(error_msg.contains("is not on the secp256k1 curve"));
}

#[test]
fn test_infinity_point() {
    // Test that the point at infinity is valid
    let p = Point::new(None, None);
    assert!(p.is_ok(), "Point at infinity should be valid");
    assert!(matches!(p.unwrap(), Point::Infinity));
}

#[test]
fn test_invalid_point_mixed_none() {
    // Test that points with only one coordinate (x or y) are invalid
    let x = FieldElement::new(BigInt::from(1)).unwrap();
    let p1 = Point::new(Some(x.clone()), None);
    assert!(
        p1.is_err(),
        "Point with only x coordinate should be invalid"
    );
    let p2 = Point::new(None, Some(x));
    assert!(
        p2.is_err(),
        "Point with only y coordinate should be invalid"
    );
}

//--------------
// Display Tests
//--------------

#[test]
fn test_display_valid_point() {
    // Test that the Display implementation formats a valid point correctly
    let correct_str = "Point(x=0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798, y=0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8)";
    assert_eq!(G.to_string(), correct_str);
}

#[test]
fn test_display_infinity_point() {
    // Test that the Display implementation formats the point at infinity correctly
    let p = Point::new(None, None).unwrap();
    assert_eq!(p.to_string(), "Point(Infinity)");
}

//---------------
// Addition Tests
//---------------

#[test]
fn test_point_addition_distinct_points() {
    // Test addition of two distinct points: G + 2G = 3G
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let three_g = point_from_hex(
        "f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
        "388f7b0f632de8140fe337e62a37f3566500a99934c2231b6cb9fd7584b8e672",
    );
    let sum = &*G + &two_g;
    assert_ne!(
        sum,
        Point::Infinity,
        "Sum of distinct points should not be infinity"
    );
    assert_eq!(sum, three_g);
}

#[test]
fn test_point_doubling_generator() {
    // Test doubling the generator point: G + G = 2G
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let double_g = &*G + &*G;
    assert_eq!(double_g, two_g);
}

#[test]
fn test_point_doubling_y_zero() {
    // Test doubling a point with y=0 returns infinity
    let x = FieldElement::new(BigInt::from(1)).unwrap();
    let y = FieldElement::zero();
    let p = Point::new(Some(x), Some(y)).unwrap_or(Point::Infinity);
    let double_p = &p + &p;
    assert_eq!(
        double_p,
        Point::Infinity,
        "Doubling a point with y = 0 should yield infinity"
    );
}

#[test]
fn test_point_addition_inverse() {
    // Test that adding a point to its inverse yields infinity: P + (-P) = ∞
    let neg_y = -G.y();
    let neg_p = Point::new(Some(G.x().clone()), Some(neg_y)).unwrap();
    let sum = &*G + &neg_p;
    assert_eq!(sum, Point::Infinity);
}

#[test]
fn test_point_addition_commutative() {
    // Test that point addition is commutative: P + Q = Q + P
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let sum1 = &*G + &two_g;
    let sum2 = &two_g + &*G;
    assert_eq!(sum1, sum2);
}

#[test]
fn test_add_to_infinity() {
    // Test addition with infinity: P + ∞ = P, ∞ + P = P, ∞ + ∞ = ∞
    let infinity = Point::new(None, None).unwrap();
    assert_eq!(&*G + &infinity, *G, "P + ∞ should equal P");
    assert_eq!(&infinity + &*G, *G, "∞ + P should equal P");
    assert_eq!(&infinity + &infinity, infinity, "∞ + ∞ should equal ∞");
}

#[test]
fn test_add_associativity() {
    // Test that point addition is associative: (P + Q) + R = P + (Q + R)
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let four_g = point_from_hex(
        "e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd13",
        "51ed993ea0d455b75642e2098ea51448d967ae33bfbdfe40cfe97bdc47739922",
    );
    let result1 = &(&*G + &*G) + &two_g;
    let result2 = &*G + &(&*G + &two_g);
    assert_eq!(result1, four_g);
    assert_eq!(result2, four_g);
}

#[test]
fn test_add_owned() {
    // Test addition with owned values: G + G = 2G
    let g = G.clone();
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    assert_eq!(g.clone() + g, two_g);
}

//----------------------------
// Scalar Multiplication Tests
//----------------------------

#[test]
fn test_scalar_mul_zero() {
    // Test that 0 * G = ∞
    let result = &*G * &BigInt::zero();
    assert_eq!(result, Point::Infinity);
}

#[test]
fn test_scalar_mul_one() {
    // Test that 1 * G = G
    let result = &*G * &BigInt::one();
    assert_eq!(result, *G);
}

#[test]
fn test_scalar_mul_two() {
    // Test that 2 * G = G + G = 2G
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let result = &*G * &BigInt::from(2);
    assert_eq!(result, two_g);
}

#[test]
fn test_scalar_mul_three() {
    // Test that 3 * G = G + G + G = 3G
    let three_g = point_from_hex(
        "f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
        "388f7b0f632de8140fe337e62a37f3566500a99934c2231b6cb9fd7584b8e672",
    );
    let result = &*G * &BigInt::from(3);
    assert_eq!(result, three_g);
}

#[test]
fn test_scalar_mul_n() {
    // Test that N * G = ∞ (group order)
    let result = &*G * &*SECP256K1_N;
    assert_eq!(result, Point::Infinity);
}

#[test]
fn test_scalar_mul_large_k() {
    // Test that (N + 2) * G = 2 * G (modulo N)
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let large_k = &*SECP256K1_N + BigInt::from(2);
    let result = &*G * &large_k;
    assert_eq!(result, two_g);
}

#[test]
fn test_scalar_mul_negative() {
    // Test that -1 * G = (N - 1) * G
    let neg_one_result = &*G * &BigInt::from(-1);
    let n_minus_one = &*SECP256K1_N - BigInt::one();
    let n_minus_one_result = &*G * &n_minus_one;
    assert_eq!(neg_one_result, n_minus_one_result);
}

#[test]
fn test_scalar_mul_infinity() {
    // Test that n * ∞ = ∞ for any n
    let infinity = Point::new(None, None).unwrap();
    let n = BigInt::from(42);
    let result = &infinity * &n;
    assert_eq!(result, Point::Infinity);
}

#[test]
fn test_scalar_mul_owned() {
    // Test scalar multiplication with owned values: 2 * G = 2G
    let g = G.clone();
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let result = g * BigInt::from(2);
    assert_eq!(result, two_g);
}

// Helper methods for coordinate access (needed for tests)
impl Point {
    pub fn x(&self) -> &FieldElement {
        match self {
            Point::Coordinates { x, .. } => x,
            Point::Infinity => panic!("Infinity has no x coordinate"),
        }
    }

    pub fn y(&self) -> &FieldElement {
        match self {
            Point::Coordinates { y, .. } => y,
            Point::Infinity => panic!("Infinity has no y coordinate"),
        }
    }
}
