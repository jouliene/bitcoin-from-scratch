use crate::finite_fields::FieldElement;
use num_bigint::BigInt;
use num_traits::{One, Zero};

//---------------------
// Constructor Tests
//---------------------

#[test]
fn test_new_valid() {
    // Test that a valid number (42) can be wrapped into a FieldElement.
    let fe = FieldElement::new(BigInt::from(42)).unwrap();
    assert_eq!(*fe.num(), BigInt::from(42));
}

#[test]
fn test_new_upper_bound() {
    // Test that the upper bound (p - 1) is valid and correctly stored.
    let p = FieldElement::prime();
    let fe = FieldElement::new(p - BigInt::one()).unwrap();
    assert_eq!(*fe.num(), p - BigInt::one());
}

#[test]
fn test_new_invalid() {
    // Test that creating a FieldElement with the prime p (invalid) returns an error.
    let p = FieldElement::prime();
    assert!(FieldElement::new(p.clone()).is_err());
    // Test that creating a FieldElement with a negative number returns an error.
    assert!(FieldElement::new(BigInt::from(-1)).is_err());
}

//---------------------
// Display Test
//---------------------

#[test]
fn test_display() {
    // Test that the Display implementation formats the FieldElement correctly as a hex string.
    let fe = FieldElement::new(BigInt::from(255)).unwrap();
    let s = format!("{}", fe);
    let expected = "FieldElement_0x00000000000000000000000000000000000000000000000000000000000000ff_(mod 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f)";
    assert_eq!(s, expected);
}

//---------------------
// Equality Test
//---------------------

#[test]
fn test_eq() {
    // Test equality and inequality of FieldElement instances based on their num values.
    let a = FieldElement::new(BigInt::from(5)).unwrap();
    let b = FieldElement::new(BigInt::from(5)).unwrap();
    let c = FieldElement::new(BigInt::from(6)).unwrap();
    assert_eq!(a, b);
    assert_ne!(a, c);
}

//---------------------
// Addition Tests
//---------------------

#[test]
fn test_add_ref_no_wraparound() {
    // Test addition of two small numbers without modular wraparound: 100 + 200 = 300.
    let a = FieldElement::new(BigInt::from(100)).unwrap();
    let b = FieldElement::new(BigInt::from(200)).unwrap();
    let c = &a + &b;
    assert_eq!(*c.num(), BigInt::from(300));
}

#[test]
fn test_add_ref_with_wraparound() {
    // Test addition causing wraparound: (p - 1) + 1 = p ≡ 0 mod p.
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e";
    let num_bigint = BigInt::parse_bytes(num_hex.as_bytes(), 16).unwrap();
    let p_minus_one = FieldElement::new(num_bigint).unwrap(); // p - 1
    let one = FieldElement::one();
    let c = &p_minus_one + &one;
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_add_owned_no_wraparound() {
    // Test addition with owned values without wraparound: 100 + 200 = 300.
    let a = FieldElement::new(BigInt::from(100)).unwrap();
    let b = FieldElement::new(BigInt::from(200)).unwrap();
    let c = a + b;
    assert_eq!(*c.num(), BigInt::from(300));
}

#[test]
fn test_add_owned_with_wraparound() {
    // Test addition with owned values causing wraparound: (p - 1) + 1 = p ≡ 0 mod p.
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e";
    let num_bigint = BigInt::parse_bytes(num_hex.as_bytes(), 16).unwrap();
    let p_minus_one = FieldElement::new(num_bigint).unwrap(); // p - 1
    let one = FieldElement::one();
    let c = p_minus_one + one;
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_add_to_prime() {
    // Test that adding two numbers summing to p results in 0: a + (p - a) ≡ 0 mod p.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let p = FieldElement::prime();
    let b = FieldElement::new(p - BigInt::from(42)).unwrap();
    let c = &a + &b;
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_add_commutative() {
    // Test that addition is commutative: a + b = b + a.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let b = FieldElement::new(BigInt::from(58)).unwrap();
    assert_eq!(&a + &b, &b + &a);
}

#[test]
fn test_add_associative() {
    // Test that addition is associative: (a + b) + c = a + (b + c).
    let a = FieldElement::new(BigInt::from(10)).unwrap();
    let b = FieldElement::new(BigInt::from(20)).unwrap();
    let c = FieldElement::new(BigInt::from(30)).unwrap();
    let left = &(&a + &b) + &c;
    let right = &a + &(&b + &c);
    assert_eq!(left, right);
}

#[test]
fn test_add_zero() {
    // Test that adding zero leaves the element unchanged: a + 0 = a.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let zero = FieldElement::zero();
    assert_eq!(&a + &zero, a);
}

//---------------------
// Subtraction Tests
//---------------------

#[test]
fn test_sub_ref_no_wraparound() {
    // Test subtraction without wraparound: 250 - 100 = 150.
    let a = FieldElement::new(BigInt::from(250)).unwrap();
    let b = FieldElement::new(BigInt::from(100)).unwrap();
    let c = &a - &b;
    assert_eq!(*c.num(), BigInt::from(150));
}

#[test]
fn test_sub_ref_with_wraparound() {
    // Test subtraction causing wraparound: 4 - 5 = -1 ≡ p - 1 mod p.
    let a = FieldElement::new(BigInt::from(4)).unwrap();
    let b = FieldElement::new(BigInt::from(5)).unwrap();
    let c = &a - &b;
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e";
    let expected = BigInt::parse_bytes(num_hex.as_bytes(),16).unwrap(); // p - 1
    assert_eq!(*c.num(), expected);
}

#[test]
fn test_sub_owned_no_wraparound() {
    // Test subtraction with owned values without wraparound: 270 - 130 = 140.
    let a = FieldElement::new(BigInt::from(270)).unwrap();
    let b = FieldElement::new(BigInt::from(130)).unwrap();
    let c = a - b;
    assert_eq!(*c.num(), BigInt::from(140));
}

#[test]
fn test_sub_owned_with_wraparound() {
    // Test subtraction with owned values causing wraparound: 4 - 5 = -1 ≡ p - 1 mod p.
    let a = FieldElement::new(BigInt::from(4)).unwrap();
    let b = FieldElement::new(BigInt::from(5)).unwrap();
    let c = a - b;
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e";
    let expected = BigInt::parse_bytes(num_hex.as_bytes(),16).unwrap(); // p - 1
    assert_eq!(*c.num(), expected);
}

#[test]
fn test_sub_self() {
    // Test that subtracting an element from itself gives zero: a - a = 0.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let c = &a - &a;
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_sub_zero() {
    // Test that subtracting zero leaves the element unchanged: a - 0 = a.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let zero = FieldElement::zero();
    assert_eq!(&a - &zero, a);
}

//---------------------
// Multiplication Tests
//---------------------

#[test]
fn test_mul_ref_no_wraparound() {
    // Test multiplication without wraparound: 5 * 10 = 50.
    let a = FieldElement::new(BigInt::from(5)).unwrap();
    let b = FieldElement::new(BigInt::from(10)).unwrap();
    let c = &a * &b;
    assert_eq!(*c.num(), BigInt::from(50));
}

#[test]
fn test_mul_ref_with_wraparound() {
    // Test multiplication causing wraparound: (p - 1) * 2 = 2p - 2 ≡ p - 2 mod p.
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e";
    let num_bigint = BigInt::parse_bytes(num_hex.as_bytes(), 16).unwrap();
    let p_minus_one = FieldElement::new(num_bigint).unwrap(); // p - 1
    let two = FieldElement::new(BigInt::from(2)).unwrap();
    let c = &p_minus_one * &two;
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2d";
    let expected = BigInt::parse_bytes(num_hex.as_bytes(),16).unwrap(); // p - 2
    assert_eq!(*c.num(), expected);
}

#[test]
fn test_mul_owned_no_wraparound() {
    // Test multiplication with owned values without wraparound: 100 * 5 = 500.
    let a = FieldElement::new(BigInt::from(100)).unwrap();
    let b = FieldElement::new(BigInt::from(5)).unwrap();
    let c = a * b;
    assert_eq!(*c.num(), BigInt::from(500));
}

#[test]
fn test_mul_owned_zero() {
    // Test multiplication by zero: (p - 1) * 0 = 0.
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e";
    let num_bigint = BigInt::parse_bytes(num_hex.as_bytes(), 16).unwrap();
    let p_minus_one = FieldElement::new(num_bigint).unwrap(); // p - 1
    let zero = FieldElement::zero();
    let c = p_minus_one * zero;
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_mul_commutative() {
    // Test that multiplication is commutative: a * b = b * a.
    let a = FieldElement::new(BigInt::from(7)).unwrap();
    let b = FieldElement::new(BigInt::from(11)).unwrap();
    assert_eq!(&a * &b, &b * &a);
}

#[test]
fn test_mul_associative() {
    // Test that multiplication is associative: (a * b) * c = a * (b * c).
    let a = FieldElement::new(BigInt::from(3)).unwrap();
    let b = FieldElement::new(BigInt::from(4)).unwrap();
    let c = FieldElement::new(BigInt::from(5)).unwrap();
    let left = &(&a * &b) * &c;
    let right = &a * &(&b * &c);
    assert_eq!(left, right);
}

#[test]
fn test_mul_one() {
    // Test that multiplying by one leaves the element unchanged: a * 1 = a.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let one = FieldElement::one();
    assert_eq!(&a * &one, a);
}

//---------------------
// Division Tests
//---------------------

#[test]
fn test_div_ref_normal() {
    // Test division of two numbers: 10 / 2 = 5.
    let a = FieldElement::new(BigInt::from(10)).unwrap();
    let b = FieldElement::new(BigInt::from(2)).unwrap();
    let c = &a / &b;
    assert_eq!(*c.num(), BigInt::from(5));
}

#[test]
fn test_div_owned_normal() {
    // Test division with owned values: 15 / 3 = 5.
    let a = FieldElement::new(BigInt::from(15)).unwrap();
    let b = FieldElement::new(BigInt::from(3)).unwrap();
    let c = a / b;
    assert_eq!(*c.num(), BigInt::from(5));
}

#[test]
fn test_div_ref_inverse() {
    // Test that dividing 1 by (p - 1) and multiplying back gives 1, verifying the inverse.
    let a = FieldElement::new(BigInt::from(1)).unwrap();
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e";
    let num_bigint = BigInt::parse_bytes(num_hex.as_bytes(), 16).unwrap();
    let b = FieldElement::new(num_bigint).unwrap(); // p - 1
    let c = &a / &b; // c = 1 / (p - 1)
    assert_eq!(*(&b * &c).num(), BigInt::one()); // b * c = (p - 1) * (1 / (p - 1)) = 1
}

#[test]
fn test_div_by_self() {
    // Test that dividing a non-zero element by itself gives 1: a / a = 1.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let c = &a / &a;
    assert_eq!(*c.num(), BigInt::one());
}

#[test]
fn test_div_zero_by_nonzero() {
    // Test that dividing zero by a non-zero element gives zero: 0 / a = 0.
    let zero = FieldElement::zero();
    let a = FieldElement::new(BigInt::from(5)).unwrap();
    let c = &zero / &a;
    assert_eq!(*c.num(), BigInt::zero());
}

#[test]
fn test_div_by_one() {
    // Test that dividing by one leaves the element unchanged: a / 1 = a.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let one = FieldElement::one();
    let c = &a / &one;
    assert_eq!(c, a);
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_div_by_zero() {
    // Test that dividing by zero triggers a panic.
    let a = FieldElement::new(BigInt::from(42)).unwrap();
    let b = FieldElement::zero();
    let _ = &a / &b; // Should panic with "Division by zero"
}

//---------------------
// Scalar Multiplication Tests (coeff * fe)
//---------------------

#[test]
fn test_scalar_mul_no_wraparound() {
    // Test scalar multiplication without wraparound: 3 * 5 = 15.
    let fe = FieldElement::new(BigInt::from(5)).unwrap();
    let coeff = BigInt::from(3);
    let result = coeff * &fe;
    assert_eq!(*result.num(), BigInt::from(15));
}

#[test]
fn test_scalar_mul_with_wraparound() {
    // Test scalar multiplication causing wraparound: p * 2 ≡ 0 mod p.
    let fe = FieldElement::new(BigInt::from(2)).unwrap();
    let coeff = FieldElement::prime().clone();
    let result = coeff * &fe;
    assert_eq!(*result.num(), BigInt::zero());
}

#[test]
fn test_scalar_mul_by_zero() {
    // Test scalar multiplication by zero: 0 * (p - 1) = 0.
    let num_hex = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e";
    let num_bigint = BigInt::parse_bytes(num_hex.as_bytes(), 16).unwrap();
    let fe = FieldElement::new(num_bigint).unwrap(); // p - 1
    let coeff = BigInt::zero();
    let result = coeff * &fe;
    assert_eq!(*result.num(), BigInt::zero());
}

//---------------------
// Exponentiation Tests
//---------------------

#[test]
fn test_pow_positive() {
    // Test positive exponent: 3^2 = 9.
    let fe = FieldElement::new(BigInt::from(3)).unwrap();
    let result = fe.pow(BigInt::from(2));
    assert_eq!(*result.num(), BigInt::from(9));
}

#[test]
fn test_pow_zero() {
    // Test zero exponent: a^0 = 1 for non-zero a.
    let fe = FieldElement::new(BigInt::from(42)).unwrap();
    let result = fe.pow(BigInt::zero());
    assert_eq!(*result.num(), BigInt::one());
}

#[test]
fn test_pow_negative() {
    // Test negative exponent: a^(-1) should be the inverse, so a * a^(-1) = 1.
    let fe = FieldElement::new(BigInt::from(5)).unwrap();
    let inv = fe.pow(BigInt::from(-1));
    let product = &fe * &inv;
    assert_eq!(*product.num(), BigInt::one());
}

#[test]
fn test_pow_fermat() {
    // Test Fermat's Little Theorem: a^(p-1) ≡ 1 mod p for non-zero a.
    let fe = FieldElement::new(BigInt::from(3)).unwrap();
    let p_minus_one = FieldElement::prime() - BigInt::one();
    let result = fe.pow(p_minus_one);
    assert_eq!(*result.num(), BigInt::one());
}
