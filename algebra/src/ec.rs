use core::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::field::{Field, SquareRootField};

/// An affine point on an elliptic curve
#[derive(Debug, Clone, PartialEq)]
pub struct AffinePoint<F: Field> {
    pub x:        F,
    pub y:        F,
    pub infinity: bool,
}

/// Basic operations required for an elliptic curve
pub trait EllipticCurve: Sized + Clone + Debug + PartialEq {
    /// The field over which this curve is defined
    type BaseField: Field + SquareRootField;

    /// The type representing a point on this curve
    type Point: Clone + Debug + PartialEq;

    /// Returns the identity element (point at infinity)
    fn identity() -> Self::Point;

    /// Returns a generator point for this curve
    fn generator() -> Self::Point;

    /// Returns the order of the curve (number of points)
    fn order() -> Vec<u64>;

    /// Adds two points on the curve
    fn add_points(p1: &Self::Point, p2: &Self::Point) -> Self::Point;

    /// Multiplies a point by a scalar
    fn scalar_mul(point: &Self::Point, scalar: &[u64]) -> Self::Point;

    /// Checks if a point is on the curve
    fn is_on_curve(point: &Self::Point) -> bool;
}

/// Trait for curves in Weierstrass form: y² = x³ + ax + b
pub trait WeierstrassCurve: EllipticCurve {
    /// Returns the 'a' coefficient
    fn get_a() -> Self::BaseField;

    /// Returns the 'b' coefficient
    fn get_b() -> Self::BaseField;
}

/// Trait for curves that support pairing operations
pub trait PairingCurve: EllipticCurve {
    /// The field where pairing results live
    type TargetField: Field;

    /// Compute the pairing of two points
    fn pairing(p: &Self::Point, q: &Self::Point) -> Self::TargetField;
}
