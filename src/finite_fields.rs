use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::fmt;
use std::ops::Add;

lazy_static::lazy_static! {
    /// secp256k1 prime number = 2^256 - 2^32 - 977
    static ref PRIME: BigInt = BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
        16
    ).unwrap();
}

/// Represents an element in the finite field F_p, where p is the secp256k1 prime by default.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldElement {
    num: BigInt,
}

impl FieldElement {
    /// Create a new FieldElement, ensuring 0 <= num < PRIME
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

    /// Returns the field's prime number.
    pub fn prime() -> &'static BigInt {
        &PRIME
    }

    /// Returns a reference to the internal number.
    pub fn num(&self) -> &BigInt {
        &self.num
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldElement_0x{:064x}_(mod 0x{:064x})",
            self.num, *PRIME
        )
    }
}

// Add
impl<'a, 'b> Add<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: &'b FieldElement) -> FieldElement {
        let mut sum = &self.num + &rhs.num;
        if sum >= *FieldElement::prime() {
            sum -= FieldElement::prime();
        }
        FieldElement { num: sum }
    }
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> FieldElement {
        &self + &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let result = String::from(
            "FieldElement_0x00000000000000000000000000000000000000000000000000000000000000ff_(mod 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f)",
        );
        assert_eq!(s, result);
    }

    #[test]
    fn test_add_ref() {
        let a = FieldElement::new(BigInt::from(100)).unwrap();
        let b = FieldElement::new(BigInt::from(200)).unwrap();
        let c = &a + &b;
        assert_eq!(*c.num(), BigInt::from(300));
    }

    #[test]
    fn test_add_val() {
        let a = FieldElement::new(
            BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e",
                16,
            )
            .unwrap(),
        )
        .unwrap();
        let b = FieldElement::new(BigInt::one()).unwrap();
        let c = &a + &b;
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
        let zero = FieldElement::new(BigInt::zero()).unwrap();
        assert_eq!(&a + &zero, a);
    }

    #[test]
    fn test_add_near_prime() {
        let a = FieldElement::new(
            BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2d",
                16,
            )
            .unwrap(),
        )
        .unwrap(); // p - 2
        let b = FieldElement::new(BigInt::from(3)).unwrap();
        let c = &a + &b; // (p - 2) + 3 = p + 1 ≡ 1 mod p
        assert_eq!(*c.num(), BigInt::one());
    }

    #[test]
    fn test_add_owned() {
        let a = FieldElement::new(BigInt::from(100)).unwrap();
        let b = FieldElement::new(BigInt::from(200)).unwrap();
        let c = a + b;
        assert_eq!(*c.num(), BigInt::from(300));
    }
}
