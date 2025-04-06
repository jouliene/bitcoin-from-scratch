use crate::finite_fields::FieldElement;
use num_bigint::BigInt;
use num_traits::Zero;
use std::fmt;

// Curve equation y^2 = x^3 + ax + b
// Define constants for secp256k1 curve a = 0 and b = 7
lazy_static::lazy_static! {
    pub static ref SECP256K1_A: FieldElement = FieldElement::new(BigInt::zero()).unwrap();
    pub static ref SECP256K1_B: FieldElement = FieldElement::new(BigInt::from(7)).unwrap();
}

/// Represents a point on the secp256k1 elliptic curve.
/// The point at infinity is represented by `None` for both x and y.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
}

impl Point {
    /// Constructs a new `Point` on secp256k1: y^2 = x^3 + 7.
    /// If x and y are provided, the point must satisfy the curve equation.
    /// Use None for both x and y to represent the point at infinity.
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>) -> Result<Self, String> {
        match (x.as_ref(), y.as_ref()) {
            (Some(x_val), Some(y_val)) => {
                // Check that the point lies on the curve: y^2 = x^3 + 7 (since a = 0 for secp256k1).
                let left = y_val.pow(BigInt::from(2)); // Compute y^2
                let right = &x_val.pow(BigInt::from(3)) + &*SECP256K1_B; // Compute x^3 + b

                if left != right {
                    return Err(format!(
                        "Point (0x{:064x}, 0x{:064x}) is not on the secp256k1 curve: 0x{:064x} != 0x{:064x}",
                        x_val.num(),
                        y_val.num(),
                        left.num(),
                        right.num()
                    ));
                }
            }
            (None, None) => {
                // Point at infinity is valid, no further checks needed
            }
            (Some(_), None) | (None, Some(_)) => {
                // One coordinate is None and the other is Some - this is invalid
                return Err(
                    "Invalid point: both coordinates must be Some or both must be None".to_string(),
                );
            }
        }

        Ok(Point { x, y })
    }

    /// Creates a point at infinity.
    pub fn infinity() -> Self {
        Point::new(None, None).unwrap()
    }

    /// Checks if the point is the point at infinity.
    pub fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    /// Adds two points on the secp256k1 elliptic curve.
    pub fn add(&self, other: &Point) -> Point {
        if self.is_infinity() {
            return other.clone();
        }
        if other.is_infinity() {
            return self.clone();
        }

        let x1 = self.x.as_ref().unwrap();
        let y1 = self.y.as_ref().unwrap();
        let x2 = other.x.as_ref().unwrap();
        let y2 = other.y.as_ref().unwrap();

        if x1 == x2 {
            if y1 == y2 {
                // Point doubling: P + P
                if *y1 == FieldElement::zero() {
                    return Point::infinity();
                }
                // s = (3 * x1^2) / (2 * y1)
                // x3 = s^2 - 2*x1
                // y3 = s * (x1 - x3) - y1
                let three = FieldElement::new(BigInt::from(3)).unwrap();
                let two = FieldElement::new(BigInt::from(2)).unwrap();

                let x1_squared = x1.pow(BigInt::from(2));
                let numerator = &three * &x1_squared;
                let denominator = &two * y1;
                let s = &numerator / &denominator;

                let s_squared = s.pow(BigInt::from(2));
                let two_x1 = x1 + x1;
                let x3 = &s_squared - &two_x1;

                let x1_minus_x3 = x1 - &x3;
                let s_times_diff = &s * &x1_minus_x3;
                let y3 = &s_times_diff - y1;

                return Point::new(Some(x3), Some(y3)).unwrap();
            } else {
                // P + (-P) = infinity
                return Point::infinity();
            }
        }

        // Regular addition: P + Q where P ≠ Q
        // s = (y2 - y1) / (x2 - x1)
        // x3 = s^2 - x1 - x2
        // y3 = s * (x1 - x3) - y1
        let numerator = y2 - y1;
        let denominator = x2 - x1;
        let s = &numerator / &denominator;

        let s_squared = s.pow(BigInt::from(2));
        let s_squared_minus_x1 = &s_squared - x1;
        let x3 = &s_squared_minus_x1 - x2;

        let x1_minus_x3 = x1 - &x3;
        let s_times_diff = &s * &x1_minus_x3;
        let y3 = &s_times_diff - y1;

        Point::new(Some(x3), Some(y3)).unwrap()
    }
}

/// Formats a `Point` as a string for display purposes.
impl fmt::Display for Point {
    /// Formats the point:
    /// - If both coordinates are `Some`, prints them in hexadecimal.
    /// - If both are `None`, prints "Point(Infinity)".
    /// - If only one is `None`, prints "Invalid Point".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.x.as_ref(), self.y.as_ref()) {
            (None, None) => write!(f, "Point(Infinity)"),
            (Some(x_val), Some(y_val)) => write!(
                f,
                "Point(x=0x{:064x}, y=0x{:064x})",
                x_val.num(),
                y_val.num()
            ),
            _ => write!(f, "Invalid Point"),
        }
    }
}
