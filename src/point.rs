use crate::finite_fields::FieldElement;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, Mul};

// Curve equation y^2 = x^3 + ax + b
// Constants for secp256k1 curve are a = 0 and b = 7
// Thus secp256k1 curve becomes y^2 = x^3 + 7
lazy_static::lazy_static! {
    // b = 7 in secp256k1 curve y^2 = x^3 + 7
    pub static ref SECP256K1_B: FieldElement = FieldElement::new(BigInt::from(7)).unwrap();

    // just field_element = 2 for short usage
    pub static ref TWO: FieldElement = FieldElement::new(BigInt::from(2)).unwrap();

    // just field_element = 3 for short usage
    pub static ref THREE: FieldElement = FieldElement::new(BigInt::from(3)).unwrap();

    // Group Order N in secp256k1 curve: N * G = Point::Infinity
    pub static ref SECP256K1_N: BigInt = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();

    // Generator point G in secp256k1 curve: N * G = Point::Infinity
    pub static ref G: Point = {
        let x = FieldElement::new(BigInt::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap()).unwrap();
        let y = FieldElement::new(BigInt::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap()).unwrap();
        Point::new(Some(x), Some(y)).unwrap()
    };
}

/// Represents a point on the secp256k1 elliptic curve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Point {
    Infinity,
    Coordinates { x: FieldElement, y: FieldElement },
}

impl Point {
    /// Constructs a new `Point` on secp256k1: y^2 = x^3 + 7.
    /// If both x and y are None, returns the point at infinity.
    /// If both are Some, validates the curve equation.
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>) -> Result<Self, String> {
        match (x, y) {
            // Point at infinity
            (None, None) => Ok(Point::Infinity),

            // Regular point with both coordinates
            (Some(x), Some(y)) => {
                // Check that the point lies on the curve: y^2 = x^3 + 7
                let left = y.pow(BigInt::from(2));
                let right = &x.pow(BigInt::from(3)) + &*SECP256K1_B;
                if left == right {
                    Ok(Point::Coordinates { x, y })
                } else {
                    Err(format!(
                        "Point ({}, {}) is not on the secp256k1 curve",
                        x, y
                    ))
                }
            }
            _ => {
                Err("Invalid point: both coordinates must be Some or both must be None".to_string())
            }
        }
    }

    /// Performs point doubling: P + P on the secp256k1 elliptic curve.
    fn point_double(&self) -> Point {
        if let Point::Coordinates { x, y } = self {
            if y == &FieldElement::zero() {
                return Point::Infinity;
            }

            // s = (3 * x^2) / (2 * y)
            let numerator = &*THREE * &(x.pow(BigInt::from(2)));
            let denominator = &*TWO * y;
            let s = &numerator / &denominator;

            // x3 = s^2 - 2*x
            let x3 = &(s.pow(BigInt::from(2))) - &(x + x);

            // y3 = s * (x - x3) - y
            let x_minus_x3 = x - &x3;
            let s_times_diff = &s * &x_minus_x3;
            let y3 = &s_times_diff - y;

            Point::Coordinates { x: x3, y: y3 }
        } else {
            Point::Infinity
        }
    }

    /// Performs point addition for two distinct points P + Q where P ≠ Q on the secp256k1 elliptic curve.
    fn point_add_distinct(&self, other: &Point) -> Point {
        if let (Point::Coordinates { x: x1, y: y1 }, Point::Coordinates { x: x2, y: y2 }) =
            (self, other)
        {
            // s = (y2 - y1) / (x2 - x1)
            let s = &(y2 - y1) / &(x2 - x1);

            // x3 = s^2 - x1 - x2
            let s_squared = s.pow(BigInt::from(2));
            let s_squared_minus_x1 = &s_squared - x1;
            let x3 = &s_squared_minus_x1 - x2;

            // y3 = s * (x1 - x3) - y1
            let x1_minus_x3 = x1 - &x3;
            let s_times_diff = &s * &x1_minus_x3;
            let y3 = &s_times_diff - y1;

            Point::Coordinates { x: x3, y: y3 }
        } else {
            Point::Infinity
        }
    }
}

/// Formats a `Point` as a string for display purposes.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Point::Infinity => write!(f, "Point(Infinity)"),
            Point::Coordinates { x, y } => {
                write!(f, "Point(x=0x{:064x}, y=0x{:064x})", x.num(), y.num())
            }
        }
    }
}

/// Implement Add for references to Point
impl<'a> Add<&'a Point> for &Point {
    type Output = Point;
    fn add(self, rhs: &'a Point) -> Point {
        match (self, rhs) {
            (Point::Infinity, _) => rhs.clone(),
            (_, Point::Infinity) => self.clone(),
            (Point::Coordinates { x: x1, y: y1 }, Point::Coordinates { x: x2, y: y2 }) => {
                if x1 == x2 {
                    if y1 == y2 {
                        self.point_double() // P + P
                    } else {
                        Point::Infinity // P + (-P) = infinity
                    }
                } else {
                    self.point_add_distinct(rhs) // P + Q
                }
            }
        }
    }
}

/// Implement Add for owned Point values
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        &self + &rhs
    }
}

/// Implement Mul for references to Point and BigInt
impl Mul<&BigInt> for &Point {
    type Output = Point;
    fn mul(self, rhs: &BigInt) -> Point {
        let mut k = rhs % &*SECP256K1_N;
        if k < BigInt::zero() {
            k += &*SECP256K1_N; // Handle negative scalars
        }

        let mut result = Point::Infinity;
        let mut current = self.clone();

        while k > BigInt::zero() {
            if &k & BigInt::one() == BigInt::one() {
                result = &result + &current;
            }
            current = &current + &current;
            k >>= 1;
        }
        result
    }
}

/// Implement Mul for owned Point and BigInt
impl Mul<BigInt> for Point {
    type Output = Point;

    fn mul(self, rhs: BigInt) -> Point {
        // Delegate to the reference version
        &self * &rhs
    }
}
