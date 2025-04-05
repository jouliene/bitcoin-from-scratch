use crate::finite_fields::FieldElement;
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[test]
fn test_new_valid() {
    let fe = FieldElement::new(BigInt::from(42)).unwrap();
    assert_eq!(*fe.num(), BigInt::from(42));
}

#[test]
fn test_new_invalid() {
    let p = FieldElement::prime();
    assert!(FieldElement::new(p.clone()).is_err());
    assert!(FieldElement::new(BigInt::from(-1)).is_err());
}

#[test]
fn test_display() {
    let fe = FieldElement::new(BigInt::from(255)).unwrap();
    let s = format!("{}", fe);
    let expected = "FieldElement_0x00000000000000000000000000000000000000000000000000000000000000ff_(mod 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f)";
    assert_eq!(s, expected);
}

//----------
// ADDITION
//----------

#[test]
fn test_add_ref_normal() {
    let a = FieldElement::new(BigInt::from(100)).unwrap();
    let b = FieldElement::new(BigInt::from(200)).unwrap();
    let c = &a + &b;
    assert_eq!(*c.num(), BigInt::from(300));
}

#[test]
fn test_add_ref_wraparound() {
    let a = FieldElement::new(
        BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e",
            16,
        )
        .unwrap(),
    )
    .unwrap(); // p - 1
    let b = FieldElement::one();
    let c = &a + &b; // (p - 1) + 1 = p ≡ 0 mod p
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_add_owned_normal() {
    let a = FieldElement::new(BigInt::from(100)).unwrap();
    let b = FieldElement::new(BigInt::from(200)).unwrap();
    let c = a + b;
    assert_eq!(*c.num(), BigInt::from(300));
}

#[test]
fn test_add_owned_wraparound() {
    let a = FieldElement::new(
        BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e",
            16,
        )
        .unwrap(),
    )
    .unwrap(); // p - 1
    let b = FieldElement::one();
    let c = a + b; // (p - 1) + 1 = p ≡ 0 mod p
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_add_commutative() {
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let b = FieldElement::new(BigInt::from(58)).unwrap();
    assert_eq!(&a + &b, &b + &a);
}

#[test]
fn test_add_associative() {
    let a = FieldElement::new(BigInt::from(10)).unwrap();
    let b = FieldElement::new(BigInt::from(20)).unwrap();
    let c = FieldElement::new(BigInt::from(30)).unwrap();
    let left = &(&a + &b) + &c;
    let right = &a + &(&b + &c);
    assert_eq!(left, right);
}

#[test]
fn test_add_zero() {
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let zero = FieldElement::zero();
    assert_eq!(&a + &zero, a);
}

//-----------
// SUBTRACTION
//-----------

#[test]
fn test_sub_ref_normal() {
    let a = FieldElement::new(BigInt::from(250)).unwrap();
    let b = FieldElement::new(BigInt::from(100)).unwrap();
    let c = &a - &b;
    assert_eq!(*c.num(), BigInt::from(150));
}

#[test]
fn test_sub_ref_wraparound() {
    let a = FieldElement::new(BigInt::from(4)).unwrap();
    let b = FieldElement::new(BigInt::from(5)).unwrap();
    let c = &a - &b; // 4 - 5 = p - 1
    let expected = BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e",
        16,
    )
    .unwrap(); // p - 1
    assert_eq!(*c.num(), expected);
}

#[test]
fn test_sub_owned_normal() {
    let a = FieldElement::new(BigInt::from(270)).unwrap();
    let b = FieldElement::new(BigInt::from(130)).unwrap();
    let c = a - b;
    assert_eq!(*c.num(), BigInt::from(140));
}

#[test]
fn test_sub_owned_wraparound() {
    let a = FieldElement::new(BigInt::from(4)).unwrap();
    let b = FieldElement::new(BigInt::from(5)).unwrap();
    let c = a - b; // 4 - 5 = p - 1
    let expected = BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e",
        16,
    )
    .unwrap(); // p - 1
    assert_eq!(*c.num(), expected);
}

#[test]
fn test_sub_zero() {
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let zero = FieldElement::zero();
    assert_eq!(&a - &zero, a);
}

//---------------
// MULTIPLICATION
//---------------

#[test]
fn test_mul_ref_normal() {
    let a = FieldElement::new(BigInt::from(5)).unwrap();
    let b = FieldElement::new(BigInt::from(10)).unwrap();
    let c = &a * &b;
    assert_eq!(*c.num(), BigInt::from(50));
}

#[test]
fn test_mul_ref_wraparound() {
    let a = FieldElement::new(
        BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e",
            16,
        )
        .unwrap(),
    )
    .unwrap(); // p - 1
    let b = FieldElement::new(BigInt::from(2)).unwrap();
    let c = &a * &b; // (p - 1) * 2 = p - 2
    let expected = BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2d",
        16,
    )
    .unwrap(); // p - 2
    assert_eq!(*c.num(), expected);
}

#[test]
fn test_mul_owned_normal() {
    let a = FieldElement::new(BigInt::from(100)).unwrap();
    let b = FieldElement::new(BigInt::from(5)).unwrap();
    let c = a * b;
    assert_eq!(*c.num(), BigInt::from(500));
}

#[test]
fn test_mul_owned_zero() {
    let a = FieldElement::new(
        BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e",
            16,
        )
        .unwrap(),
    )
    .unwrap(); // p - 1
    let b = FieldElement::zero();
    let c = a * b;
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_mul_commutative() {
    let a = FieldElement::new(BigInt::from(7)).unwrap();
    let b = FieldElement::new(BigInt::from(11)).unwrap();
    assert_eq!(&a * &b, &b * &a);
}

#[test]
fn test_mul_associative() {
    let a = FieldElement::new(BigInt::from(3)).unwrap();
    let b = FieldElement::new(BigInt::from(4)).unwrap();
    let c = FieldElement::new(BigInt::from(5)).unwrap();
    let left = &(&a * &b) * &c;
    let right = &a * &(&b * &c);
    assert_eq!(left, right);
}

#[test]
fn test_mul_one() {
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let one = FieldElement::one();
    assert_eq!(&a * &one, a);
}

//----------
// DIVISION
//----------

#[test]
fn test_div_ref_normal() {
    let a = FieldElement::new(BigInt::from(10)).unwrap();
    let b = FieldElement::new(BigInt::from(2)).unwrap();
    let c = &a / &b; // 10 / 2 = 5
    assert_eq!(*c.num(), BigInt::from(5));
}

#[test]
fn test_div_owned_normal() {
    let a = FieldElement::new(BigInt::from(15)).unwrap();
    let b = FieldElement::new(BigInt::from(3)).unwrap();
    let c = a / b; // 15 / 3 = 5
    assert_eq!(*c.num(), BigInt::from(5));
}

#[test]
fn test_div_ref_inverse() {
    let a = FieldElement::new(BigInt::from(1)).unwrap();
    let b = FieldElement::new(
        BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e",
            16,
        )
        .unwrap(),
    )
    .unwrap(); // p - 1
    let c = &a / &b; // 1 / (p - 1)
    assert_eq!(*(&b * &c).num(), BigInt::one()); // b * (1/b) = 1
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_div_by_zero() {
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let b = FieldElement::zero();
    let _ = &a / &b; // Should panic
}
