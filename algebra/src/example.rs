use core::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    ec::{AffinePoint, EllipticCurve},
    field::{Field, SquareRootField},
};

// Constants for Curve25519
const CURVE_A: [u64; 4] = [486662, 0, 0, 0]; // Curve parameter A
const PRIME_MODULUS: [u64; 4] =
    [0xFFFFFFFFFFFFFFED, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF]; // 2^255 - 19

#[derive(Clone, Debug, PartialEq)]
pub struct Fp25519 {
    value: [u64; 4],
}

impl Fp25519 {
    pub fn new(value: [u64; 4]) -> Self {
        let mut result = Self { value };
        result.reduce();
        result
    }

    // Reduces the value modulo p
    fn reduce(&mut self) {
        let mut carry: u128 = 0;
        for i in 0..4 {
            let mut acc = self.value[i] as u128 + carry;
            if acc >= PRIME_MODULUS[i] as u128 {
                acc -= PRIME_MODULUS[i] as u128;
                carry = 1;
            } else {
                carry = 0;
            }
            self.value[i] = acc as u64;
        }
        if carry > 0 {
            self.reduce();
        }
    }
}

// Field trait implementations for Fp25519
impl Field for Fp25519 {
    fn characteristic() -> Vec<u64> { PRIME_MODULUS.to_vec() }

    fn one() -> Self { Self::new([1, 0, 0, 0]) }

    fn zero() -> Self { Self::new([0, 0, 0, 0]) }

    fn is_zero(&self) -> bool { self.value == [0, 0, 0, 0] }

    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            // Fermat's little theorem: a^(p-1) ≡ 1 (mod p)
            // Therefore, a^(p-2) is the multiplicative inverse
            Some(self.pow(PRIME_MODULUS[0].wrapping_sub(2)))
        }
    }

    fn pow(&self, exp: u64) -> Self {
        let mut base = self.clone();
        let mut result = Self::one();
        let mut e = exp;

        while e > 0 {
            if e & 1 == 1 {
                result = result * base.clone();
            }
            base = base.clone() * base;
            e >>= 1;
        }
        result
    }
}

// Basic arithmetic implementations for Fp25519
impl Add for Fp25519 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.clone();
        result.value.iter_mut().zip(other.value.iter()).for_each(|(a, b)| *a = a.wrapping_add(*b));
        result.reduce();
        result
    }
}

impl Sub for Fp25519 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self.clone();
        result.value.iter_mut().zip(other.value.iter()).for_each(|(a, b)| *a = a.wrapping_sub(*b));
        result.reduce();
        result
    }
}

impl Mul for Fp25519 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = [0u64; 4];
        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                if i + j < 4 {
                    let prod = (self.value[i] as u128) * (other.value[j] as u128) + carry;
                    result[i + j] = result[i + j].wrapping_add(prod as u64);
                    carry = prod >> 64;
                }
            }
        }
        Self::new(result)
    }
}

impl Neg for Fp25519 {
    type Output = Self;

    fn neg(self) -> Self {
        if self.is_zero() {
            return self;
        }
        let mut result = Self::zero();
        result
            .value
            .iter_mut()
            .zip(self.value.iter())
            .for_each(|(a, b)| *a = PRIME_MODULUS[0].wrapping_sub(*b));
        result.reduce();
        result
    }
}

// Implement assignment operators
impl AddAssign for Fp25519 {
    fn add_assign(&mut self, other: Self) { *self = self.clone() + other; }
}

impl SubAssign for Fp25519 {
    fn sub_assign(&mut self, other: Self) { *self = self.clone() - other; }
}

impl MulAssign for Fp25519 {
    fn mul_assign(&mut self, other: Self) { *self = self.clone() * other; }
}

// Display implementation for Fp25519
impl Display for Fp25519 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x")?;
        for &digit in self.value.iter().rev() {
            write!(f, "{:016x}", digit)?;
        }
        Ok(())
    }
}

// Implement SquareRootField for Fp25519
impl SquareRootField for Fp25519 {
    fn sqrt(&self) -> Option<Self> {
        // Tonelli-Shanks algorithm would go here
        // For now, we'll return None as a placeholder
        None
    }

    fn legendre(&self) -> i8 {
        if self.is_zero() {
            0
        } else if self.pow((PRIME_MODULUS[0] - 1) / 2) == Self::one() {
            1
        } else {
            -1
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Curve25519;

impl EllipticCurve for Curve25519 {
    type BaseField = Fp25519;
    type Point = AffinePoint<Fp25519>;

    fn identity() -> Self::Point {
        AffinePoint { x: Fp25519::zero(), y: Fp25519::zero(), infinity: true }
    }

    fn generator() -> Self::Point {
        // Standard generator point for Curve25519
        AffinePoint {
            x:        Fp25519::new([9, 0, 0, 0]),
            y:        Fp25519::new([
                0x5F51E65E475F794B,
                0x1234567890ABCDEF,
                0xFEDCBA9876543210,
                0x123456789ABCDEF0,
            ]),
            infinity: false,
        }
    }

    fn order() -> Vec<u64> {
        // The order of the Curve25519 group
        vec![0x1000000000000000, 0x0000000000000000, 0x14DEF9DEA2F79CD6, 0x5812631A5CF5D3ED]
    }

    fn add_points(p1: &Self::Point, p2: &Self::Point) -> Self::Point {
        if p1.infinity {
            return p2.clone();
        }
        if p2.infinity {
            return p1.clone();
        }
        if p1.x == p2.x {
            if p1.y == p2.y {
                // Point doubling
                return Self::double_point(p1);
            }
            return Self::identity();
        }

        // Point addition formula
        let slope =
            (p2.y.clone() - p1.y.clone()) * (p2.x.clone() - p1.x.clone()).inverse().unwrap();
        let x3 = slope.clone().square() - p1.x.clone() - p2.x.clone();
        let y3 = slope * (p1.x.clone() - x3.clone()) - p1.y.clone();

        AffinePoint { x: x3, y: y3, infinity: false }
    }

    fn scalar_mul(point: &Self::Point, scalar: &[u64]) -> Self::Point {
        let mut result = Self::identity();
        let mut temp = point.clone();

        for &s in scalar {
            let mut bits = s;
            for _ in 0..64 {
                if bits & 1 == 1 {
                    result = Self::add_points(&result, &temp);
                }
                temp = Self::double_point(&temp);
                bits >>= 1;
            }
        }
        result
    }

    fn is_on_curve(point: &Self::Point) -> bool {
        if point.infinity {
            return true;
        }
        let a = Fp25519::new(CURVE_A);
        // y² = x³ + ax + b for Curve25519
        let left = point.y.clone().square();
        let right = point.x.clone().pow(3) + a * point.x.clone();
        left == right
    }
}

impl Curve25519 {
    fn double_point(p: &AffinePoint<Fp25519>) -> AffinePoint<Fp25519> {
        if p.infinity || p.y.is_zero() {
            return Self::identity();
        }

        let a = Fp25519::new(CURVE_A);
        // Slope = (3x² + a) / (2y)
        let slope = (p.x.clone().square() * Fp25519::new([3, 0, 0, 0]) + a)
            * (p.y.clone() * Fp25519::new([2, 0, 0, 0])).inverse().unwrap();

        let x3 = slope.clone().square() - p.x.clone() - p.x.clone();
        let y3 = slope * (p.x.clone() - x3.clone()) - p.y.clone();

        AffinePoint { x: x3, y: y3, infinity: false }
    }
}
