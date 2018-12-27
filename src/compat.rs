use core::fmt::{self, Display};

use Error;
use Fail;

/// A compatibility wrapper around an error type from this crate.
///
/// `Compat` implements `std::error::Error`, allowing the types from this
/// crate to be passed to interfaces that expect a type of that trait.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Compat<E> {
	pub(crate) error: E,
}

impl<E: Display> Display for Compat<E> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Display::fmt(&self.error, f)
	}
}

impl<E> Compat<E> {
	/// Unwraps this into the inner error.
	pub fn into_inner(self) -> E {
		self.error
	}

	/// Gets a reference to the inner error.
	pub fn get_ref(&self) -> &E {
		&self.error
	}
}

impl Fail for Compat<Error> {
	fn backtrace(&self) -> Option<&crate::Backtrace> {
		Some(self.error.backtrace())
	}
}

with_std! {
	use std::fmt::Debug;
	use std::error::Error as StdError;

	impl<E: Display + Debug> StdError for Compat<E> {
		fn description(&self) -> &'static str {
			"An error has occurred."
		}
	}

	impl From<Error> for Box<StdError> {
		fn from(error: Error) -> Box<StdError> {
			Box::new(Compat { error })
		}
	}
}
