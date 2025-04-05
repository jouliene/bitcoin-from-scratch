use crate::finite_fields::FieldElement;
use crate::point::Point;
use num_bigint::BigInt;
use std::string::ToString;

#[test]
fn test_valid_point() {
    // Test that the secp256k1 generator point G is valid.
    let x_hex = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"; // Generator x
    let y_hex = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"; // Generator y

    // Parse the hexadecimal strings into BigInt values.
    let x_bigint = BigInt::parse_bytes(x_hex.as_bytes(), 16).unwrap();
    let y_bigint = BigInt::parse_bytes(y_hex.as_bytes(), 16).unwrap();

    // Convert to FieldElement values.
    let x = FieldElement::new(x_bigint).unwrap();
    let y = FieldElement::new(y_bigint).unwrap();

    // Create the point and verify it’s valid.
    let p = Point::new(Some(x), Some(y));
    assert!(p.is_ok(), "Generator point should satisfy y^2 = x^3 + 7");
}

#[test]
fn test_invalid_point() {
    // Test that an invalid point (1, 2) does not satisfy y^2 = x^3 + 7.
    let x = FieldElement::new(BigInt::from(1)).unwrap(); // x = 1
    let y = FieldElement::new(BigInt::from(2)).unwrap(); // y = 2
    let p = Point::new(Some(x), Some(y)); // Should fail: 4 ≠ 8

    // Verify the point is rejected with an appropriate error message.
    assert!(p.is_err(), "Point (1, 2) should be invalid");
    let error_msg = p.unwrap_err();
    assert!(error_msg.contains("is not on the secp256k1 curve"));
}

#[test]
fn test_infinity_point() {
    // Test that the point at infinity is valid.
    let p = Point::new(None, None); // Create point at infinity
    assert!(p.is_ok(), "Point at infinity should be valid");
}

#[test]
fn test_invalid_point_mixed_none() {
    // Test that points with only one coordinate (x or y) are invalid.
    let x = FieldElement::new(BigInt::from(1)).unwrap(); // x = 1

    // Check point with x but no y.
    let p1 = Point::new(Some(x.clone()), None);
    assert!(
        p1.is_err(),
        "Point with only x coordinate should be invalid"
    );

    // Check point with y but no x.
    let p2 = Point::new(None, Some(x));
    assert!(
        p2.is_err(),
        "Point with only y coordinate should be invalid"
    );
}

#[test]
fn test_display_valid_point() {
    // Test Display formatting for a valid point on the secp256k1 curve.
    let x_hex = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"; // Generator x
    let y_hex = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"; // Generator y

    // Parse the hexadecimal strings into BigInt values.
    let x_bigint = BigInt::parse_bytes(x_hex.as_bytes(), 16).unwrap();
    let y_bigint = BigInt::parse_bytes(y_hex.as_bytes(), 16).unwrap();

    // Convert to FieldElement values.
    let x = FieldElement::new(x_bigint).unwrap();
    let y = FieldElement::new(y_bigint).unwrap();

    // Create a valid point using the generator coordinates.
    let p = Point::new(Some(x), Some(y)).unwrap();

    // Verify the formatted string.
    let correct_str = "Point(x=0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798, y=0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8)";
    assert_eq!(p.to_string(), correct_str);
}

#[test]
fn test_display_infinity_point() {
    // Test Display formatting for the point at infinity.
    let p = Point::new(None, None).unwrap(); // Create point at infinity
    assert_eq!(p.to_string(), "Point(Infinity)");
}

#[test]
fn test_display_invalid_point() {
    // Test Display formatting for an invalid point representation (bypassing constructor).
    let x = FieldElement::new(BigInt::from(1)).unwrap(); // x = 1
    let invalid_point = Point {
        x: Some(x),
        y: None,
    }; // Directly construct invalid point

    // Verify the formatted string for invalid state.
    assert_eq!(
        invalid_point.to_string(),
        "Invalid Point",
        "Should format as 'Invalid Point'"
    );
}
