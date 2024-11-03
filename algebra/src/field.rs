use core::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Represents the basic operations required for a field element.
/// This will be our foundation for both prime fields and extension fields.
pub trait Field:
    Sized
    + Clone
    + Debug
    + Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + PartialEq {
    /// The characteristic of the field
    fn characteristic() -> Vec<u64>;

    /// Returns the multiplicative identity
    fn one() -> Self;

    /// Returns the additive identity
    fn zero() -> Self;

    /// Returns true if this element is zero
    fn is_zero(&self) -> bool;

    /// Computes the multiplicative inverse of this element, if it exists
    fn inverse(&self) -> Option<Self>;

    /// Exponentiates this element by a power represented as a u64
    fn pow(&self, exp: u64) -> Self;

    /// Squares this element
    fn square(&self) -> Self { self.clone() * self.clone() }
}

/// Represents a prime field with modular arithmetic operations
pub trait PrimeField: Field {
    /// The modulus of the field
    fn modulus() -> Vec<u64>;

    /// Constructs a field element from a u64
    fn from_u64(n: u64) -> Self;

    /// Returns the value of this field element as a bit vector
    fn to_bits(&self) -> Vec<bool>;

    /// Attempts to construct a field element from a sequence of bits
    fn from_bits(bits: &[bool]) -> Option<Self>;
}

/// A finite field that supports square root operations
pub trait SquareRootField: Field {
    /// Computes the square root of this element, if it exists
    fn sqrt(&self) -> Option<Self>;

    /// Returns the Legendre symbol of this element
    fn legendre(&self) -> i8;
}
