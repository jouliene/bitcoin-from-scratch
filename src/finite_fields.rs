use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

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
    /// Constructs a new `FieldElement`, ensuring the value is in the valid range [0, p-1].
    /// Returns an error if `num` is negative or greater than or equal to the prime modulus.
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

    /// Returns a reference to the field's prime modulus (p).
    pub fn prime() -> &'static BigInt {
        &PRIME
    }

    /// Returns a reference to the internal number representing the field element.
    pub fn num(&self) -> &BigInt {
        &self.num
    }

    /// Returns the zero element (0) in the field.
    pub fn zero() -> Self {
        FieldElement::new(BigInt::zero()).unwrap()
    }

    /// Returns the one element (1) in the field.
    pub fn one() -> Self {
        FieldElement::new(BigInt::one()).unwrap()
    }

    /// Computes the multiplicative inverse using Fermat's Little Theorem: a^(p-2) ≡ a^(-1) mod p.
    /// Panics if the element is zero, as zero has no multiplicative inverse.
    fn inverse(&self) -> Self {
        if self.num == BigInt::zero() {
            panic!("Division by zero: no multiplicative inverse exists");
        }
        let exponent = FieldElement::prime() - BigInt::from(2);
        let result = self.num.modpow(&exponent, FieldElement::prime());
        FieldElement { num: result }
    }

    /// Computes exponentiation: a^n mod p, where n is reduced modulo (p-1) per Fermat's Little Theorem.
    /// This ensures a^(p-1) ≡ 1 mod p for non-zero a, and handles negative exponents correctly.
    pub fn pow(&self, exponent: BigInt) -> Self {
        let p_minus_one = Self::prime() - BigInt::one();
        if exponent.is_negative() {
            // For negative exponents, compute the inverse raised to the positive exponent
            let abs_exp = -exponent;
            let reduced_exp = abs_exp % &p_minus_one;
            self.inverse().pow(reduced_exp)
        } else {
            let reduced_exp = exponent % &p_minus_one;
            let num = self.num.modpow(&reduced_exp, Self::prime());
            FieldElement { num }
        }
    }

    /// Computes the additive inverse of the field element: -a = p - a mod p.
    pub fn negate(&self) -> FieldElement {
        let p = Self::prime();
        let neg_num = (p - &self.num) % p;
        FieldElement::new(neg_num).unwrap()
    }
}

/// Formats a `FieldElement` as a hex string with the modulus, e.g., "FieldElement_0x..._(mod 0x...)".
/// Useful for debugging and logging.
impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldElement_0x{:064x}_(mod 0x{:064x})",
            self.num, *PRIME
        )
    }
}

/// Implements addition for references to `FieldElement`, computing (a + b) mod p efficiently.
/// Avoids unnecessary modular reductions by checking if the sum exceeds p.
impl<'a> Add<&'a FieldElement> for &FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: &'a FieldElement) -> FieldElement {
        let mut result = &self.num + &rhs.num;
        if result >= *FieldElement::prime() {
            result -= FieldElement::prime();
        }
        FieldElement { num: result }
    }
}

/// Implements addition for owned `FieldElement` values, delegating to the reference version.
impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> FieldElement {
        &self + &rhs
    }
}

/// Implements subtraction for references to `FieldElement`, computing (a - b) mod p efficiently.
/// Adjusts negative results by adding p to ensure the result is in [0, p-1].
impl<'a> Sub<&'a FieldElement> for &FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: &'a FieldElement) -> FieldElement {
        let mut result = &self.num - &rhs.num;
        if result < BigInt::zero() {
            result += FieldElement::prime();
        }
        FieldElement { num: result }
    }
}

/// Implements subtraction for owned `FieldElement` values, delegating to the reference version.
impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: FieldElement) -> FieldElement {
        &self - &rhs
    }
}

/// Implements multiplication for references to `FieldElement`, computing (a * b) mod p.
/// Uses the standard approach of computing the product and then reducing modulo p.
impl<'a> Mul<&'a FieldElement> for &FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: &'a FieldElement) -> FieldElement {
        let result = (&self.num * &rhs.num) % FieldElement::prime();
        FieldElement { num: result }
    }
}

/// Implements multiplication for owned `FieldElement` values, delegating to the reference version.
impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> FieldElement {
        &self * &rhs
    }
}

/// Implements scalar multiplication for `BigInt * FieldElement`, computing (coeff * a) mod p.
impl Mul<&FieldElement> for BigInt {
    type Output = FieldElement;
    fn mul(self, rhs: &FieldElement) -> FieldElement {
        let num = (self * &rhs.num) % FieldElement::prime();
        FieldElement { num }
    }
}

/// Implements division for references to `FieldElement`, computing a / b = a * b^(-1) mod p.
/// Suppresses Clippy warning as the multiplication with inverse is intentional and correct.
#[allow(clippy::suspicious_arithmetic_impl)]
impl<'a> Div<&'a FieldElement> for &FieldElement {
    type Output = FieldElement;
    fn div(self, rhs: &'a FieldElement) -> FieldElement {
        let rhs_inv = rhs.inverse(); // Compute the inverse (owned value)
        self * &rhs_inv // Multiply reference with reference to inverse
    }
}

/// Implements division for owned `FieldElement` values, delegating to the reference version.
impl Div for FieldElement {
    type Output = FieldElement;
    fn div(self, rhs: FieldElement) -> FieldElement {
        &self / &rhs
    }
}

/// Implements the unary negation operator (-) for references to `FieldElement`.
impl Neg for &FieldElement {
    type Output = FieldElement;
    fn neg(self) -> FieldElement {
        self.negate()
    }
}

/// Implements the unary negation operator (-) for owned `FieldElement` values.
impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> FieldElement {
        self.negate()
    }
}
