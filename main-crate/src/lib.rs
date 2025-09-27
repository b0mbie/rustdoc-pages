//! This is the documentation for the main crate.

use ::another_crate::Squareable;
use ::core::{
	any::type_name,
	marker::PhantomData,
};

/// Square something that implements [`Squareable`],
/// returning [`SquareErr`] as an error if overflow occurs.
pub fn square<T: Squareable>(x: T) -> Result<T::Output, SquareErr<T>> {
	match x.checked_sqr() {
		Some(x) => Ok(x),
		None => Err(SquareErr(PhantomData)),
	}
}

/// Type for errors that may occure while [`square`]-ing a value.
/// 
/// # Implementation notes
/// The implementation is derived with [`thiserror`].
#[derive(Debug, thiserror::Error)]
#[error("overflow while squaring a value of type {ty:?}", ty = type_name::<T>())]
#[repr(transparent)]
pub struct SquareErr<T>(PhantomData<fn(T)>);
