use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::fmt;
use std::ops::Add;

// Defines the secp256k1 prime (p = 2^256 - 2^32 - 977) as a global constant.
// This is the modulus for our finite field F_p.
lazy_static::lazy_static! {
    static ref PRIME: BigInt = BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
        16
    ).unwrap();
}

/// Represents an element in the finite field F_p, where p is the secp256k1 prime.
/// Elements are integers modulo p, satisfying 0 <= num < p.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldElement {
    num: BigInt,
}

impl FieldElement {
    /// Constructs a new FieldElement, ensuring the value is in the valid range [0, p-1].
    /// Returns an error if num < 0 or num >= p.
    pub fn new(num: BigInt) -> Result<Self, String> {
        if num.is_negative() || num >= *PRIME {
            return Err(format!(
                "Number {} not in the field range 0 to {}",
                num,
                &*PRIME - BigInt::one()
            ));
        }
        Ok(FieldElement { num })
    }

    /// Returns the field's prime modulus (p).
    pub fn prime() -> &'static BigInt {
        &PRIME
    }

    /// Returns a reference to the internal number.
    pub fn num(&self) -> &BigInt {
        &self.num
    }

    /// Returns 0 in the field.
    pub fn zero() -> Self {
        FieldElement::new(Zero::zero()).unwrap()
    }

    /// Returns 1 in the field.
    pub fn one() -> Self {
        FieldElement::new(One::one()).unwrap()
    }
}

/// Formats a FieldElement as a hex string with the modulus, e.g., "FieldElement_0x..._(mod 0x...)".
impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldElement_0x{:064x}_(mod 0x{:064x})",
            self.num, *PRIME
        )
    }
}

/// Implements addition for FieldElement references, computing (a + b) mod p efficiently.
/// Avoids cloning large BigInts by operating on references.
impl<'a> Add<&'a FieldElement> for &FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: &'a FieldElement) -> FieldElement {
        let mut sum = &self.num + &rhs.num;
        if sum >= *FieldElement::prime() {
            sum -= FieldElement::prime();
        }
        FieldElement { num: sum }
    }
}

/// Implements addition for owned FieldElements, delegating to the reference version.
/// Consumes the arguments but borrows them internally for efficiency.
impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> FieldElement {
        &self + &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests constructing a valid FieldElement.
    #[test]
    fn test_new_valid() {
        let fe = FieldElement::new(BigInt::from(42)).unwrap();
        assert_eq!(*fe.num(), BigInt::from(42));
    }

    /// Tests that invalid inputs (negative or >= p) are rejected.
    #[test]
    fn test_new_invalid() {
        let p = FieldElement::prime();
        assert!(FieldElement::new(p.clone()).is_err()); // p is not in [0, p-1]
        assert!(FieldElement::new(BigInt::from(-1)).is_err()); // Negative is invalid
    }

    /// Tests the Display implementation for proper hex formatting.
    #[test]
    fn test_display() {
        let fe = FieldElement::new(BigInt::from(255)).unwrap();
        let s = format!("{}", fe);
        let expected = "FieldElement_0x00000000000000000000000000000000000000000000000000000000000000ff_(mod 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f)";
        assert_eq!(s, expected);
    }

    /// Tests reference addition without wrap-around (a + b < p).
    #[test]
    fn test_add_ref_normal() {
        let a = FieldElement::new(BigInt::from(100)).unwrap();
        let b = FieldElement::new(BigInt::from(200)).unwrap();
        let c = &a + &b;
        assert_eq!(*c.num(), BigInt::from(300));
    }

    /// Tests reference addition with wrap-around (a + b >= p).
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
        assert_eq!(*c.num(), Zero::zero());
    }

    /// Tests owned addition without wrap-around (a + b < p).
    #[test]
    fn test_add_owned_normal() {
        let a = FieldElement::new(BigInt::from(100)).unwrap();
        let b = FieldElement::new(BigInt::from(200)).unwrap();
        let c = a + b;
        assert_eq!(*c.num(), BigInt::from(300));
    }

    /// Tests owned addition with wrap-around (a + b >= p).
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
        assert_eq!(*c.num(), Zero::zero());
    }

    /// Tests that addition is commutative (a + b = b + a) using references.
    #[test]
    fn test_add_commutative() {
        let a = FieldElement::new(BigInt::from(42)).unwrap();
        let b = FieldElement::new(BigInt::from(58)).unwrap();
        assert_eq!(&a + &b, &b + &a);
    }

    /// Tests that addition is associative ((a + b) + c = a + (b + c)) using references.
    #[test]
    fn test_add_associative() {
        let a = FieldElement::new(BigInt::from(10)).unwrap();
        let b = FieldElement::new(BigInt::from(20)).unwrap();
        let c = FieldElement::new(BigInt::from(30)).unwrap();
        let left = &(&a + &b) + &c;
        let right = &a + &(&b + &c);
        assert_eq!(left, right);
    }

    /// Tests that adding zero is an identity operation (a + 0 = a).
    #[test]
    fn test_add_zero() {
        let a = FieldElement::new(BigInt::from(42)).unwrap();
        let zero = FieldElement::zero();
        assert_eq!(&a + &zero, a);
    }
}
