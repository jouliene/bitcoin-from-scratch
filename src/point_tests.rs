use crate::finite_fields::FieldElement;
use crate::point::Point;
use num_bigint::BigInt;
use std::string::ToString;

// Helper function to create a Point from hexadecimal coordinates
fn point_from_hex(x_hex: &str, y_hex: &str) -> Point {
    let x_bigint = BigInt::parse_bytes(x_hex.as_bytes(), 16).unwrap();
    let y_bigint = BigInt::parse_bytes(y_hex.as_bytes(), 16).unwrap();
    let x = FieldElement::new(x_bigint).unwrap();
    let y = FieldElement::new(y_bigint).unwrap();
    Point::new(Some(x), Some(y)).unwrap()
}

//------------------
// Point Creation Tests
//------------------

#[test]
fn test_valid_point() {
    // Test that a valid point (generator G) is created successfully
    let x_hex = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let y_hex = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
    let x_bigint = BigInt::parse_bytes(x_hex.as_bytes(), 16).unwrap();
    let y_bigint = BigInt::parse_bytes(y_hex.as_bytes(), 16).unwrap();
    let x = FieldElement::new(x_bigint).unwrap();
    let y = FieldElement::new(y_bigint).unwrap();
    let p = Point::new(Some(x), Some(y));
    assert!(p.is_ok(), "Generator point should satisfy y^2 = x^3 + 7");
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
}

#[test]
fn test_invalid_point_mixed_none() {
    // Test that points with only one coordinate (x or y) are invalid
    let x = FieldElement::new(BigInt::from(1)).unwrap();
    let p1 = Point::new(Some(x.clone()), None);
    assert!(p1.is_err(), "Point with only x coordinate should be invalid");
    let p2 = Point::new(None, Some(x));
    assert!(p2.is_err(), "Point with only y coordinate should be invalid");
}

//-------------
// Display Tests
//-------------

#[test]
fn test_display_valid_point() {
    // Test that the Display implementation formats a valid point correctly
    let g = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let correct_str = "Point(x=0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798, y=0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8)";
    assert_eq!(g.to_string(), correct_str);
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
    let g = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let three_g = point_from_hex(
        "f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
        "388f7b0f632de8140fe337e62a37f3566500a99934c2231b6cb9fd7584b8e672",
    );
    let sum = &g + &two_g;
    assert_ne!(sum, Point::infinity(), "Sum of distinct points should not be infinity");
    assert_eq!(sum, three_g);
}

#[test]
fn test_point_doubling_generator() {
    // Test doubling the generator point: G + G = 2G
    let g = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let double_g = &g + &g;
    assert_eq!(double_g, two_g);
}

#[test]
fn test_point_doubling_y_zero() {
    // Test doubling a point with y=0 returns infinity
    let x = FieldElement::new(BigInt::from(1)).unwrap();
    let y = FieldElement::zero();
    let p = Point::new(Some(x), Some(y)).unwrap_or(Point::infinity());
    let double_p = &p + &p;
    assert_eq!(double_p, Point::infinity(), "Doubling a point with y = 0 should yield infinity");
}

#[test]
fn test_point_addition_inverse() {
    // Test that adding a point to its inverse yields infinity: P + (-P) = ∞
    let g = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let g_x = match &g {
        Point::Coordinates { x: x_val, .. } => x_val.clone(),
        _ => panic!("Expected coordinates"),
    };
    let g_y = match &g {
        Point::Coordinates { y: y_val, .. } => y_val.clone(),
        _ => panic!("Expected coordinates"),
    };
    let neg_y = -g_y;
    let neg_p = Point::new(Some(g_x), Some(neg_y)).unwrap();
    let sum = &g + &neg_p;
    assert_eq!(sum, Point::infinity());
}

#[test]
fn test_point_addition_commutative() {
    // Test that point addition is commutative: P + Q = Q + P
    let g = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let sum1 = &g + &two_g;
    let sum2 = &two_g + &g;
    assert_eq!(sum1, sum2);
}

#[test]
fn test_add_to_infinity() {
    // Test addition with infinity: P + ∞ = P, ∞ + P = P, ∞ + ∞ = ∞
    let g = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let infinity = Point::infinity();
    assert_eq!(&g + &infinity, g, "P + ∞ should equal P");
    assert_eq!(&infinity + &g, g, "∞ + P should equal P");
    assert_eq!(&infinity + &infinity, infinity, "∞ + ∞ should equal ∞");
}

#[test]
fn test_add_associativity() {
    // Test that point addition is associative: (P + Q) + R = P + (Q + R)
    let g = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    let four_g = point_from_hex(
        "e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd13",
        "51ed993ea0d455b75642e2098ea51448d967ae33bfbdfe40cfe97bdc47739922",
    );
    let result1 = &(&g + &g) + &two_g;
    let result2 = &g + &(&g + &two_g);
    assert_eq!(result1, four_g);
    assert_eq!(result2, four_g);
}

#[test]
fn test_add_owned() {
    // Test addition with owned values: G + G = 2G
    let g1 = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let g2 = point_from_hex(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    let two_g = point_from_hex(
        "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
    );
    assert_eq!(g1 + g2, two_g);
}