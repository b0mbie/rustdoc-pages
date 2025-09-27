//! This is the documentation for another crate.

#![no_std]

/// Trait for arithmetic types that can be squared (raised to the power of `2`).
/// 
/// This is implemented for primitive types like [`usize`] and [`f32`].
pub trait Squareable {
	type Output;

	/// Returns the square of the value,
	/// or `None` if multiplication would overflow.
	fn checked_sqr(self) -> Option<Self::Output>;

	/// Returns the square of the value,
	/// clamping the result to the maximum value of the output type.
	fn saturating_sqr(self) -> Self::Output;
}

impl Squareable for usize {
	type Output = Self;
	fn checked_sqr(self) -> Option<Self> {
		self.checked_mul(self)
	}
	fn saturating_sqr(self) -> Self {
		self.saturating_mul(self)
	}
}

impl Squareable for f32 {
	type Output = Self;
	fn checked_sqr(self) -> Option<Self> {
		Some(self * self)
	}
	fn saturating_sqr(self) -> Self {
		self * self
	}
}
