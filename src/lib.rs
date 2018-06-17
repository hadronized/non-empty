//! Efficient non-empty vectors.
//!
//! This module provides an implementation of non-empty vectors with contiguous guarantees. That
//! is, typical implementation of non-empty lists or non-empty vectors are based on a very famous
//! implementation ([one you can find in Haskell](https://hackage.haskell.org/package/semigroups-0.18.1/docs/Data-List-NonEmpty.html#t:NonEmpty)
//! for instance) that break apart the first element of the container from the rest. That is akin
//! to the following snippet:
//!
//! ```
//! struct NonEmpty<T>(T, Vec<T>);
//! ```
//!
//! While that might seem completely sound, it makes that `NonEmpty` impossible to use in a
//! contiguous context (so don’t even think about passing this to a FFI function or assume anything
//! about its memory layout) and it’s very likely cache-friendliness will be impacted as well.
//!
//! The implementation provided by this module has all data (including the first element) in the
//! same memory region, providing the exact same cache and runtime performance as a regular `Vec`.

/// A non-empty vector.
pub struct NonEmpty<T>(Vec<T>);

impl<T> NonEmpty<T> {
  /// Construct a non-empty vector from a `Vec<T>`.
  ///
  /// This function fails if the input vector is empty.
  pub fn from_vec(vec: Vec<T>) -> Option<Self> {
    if vec.is_empty() {
      None
    } else {
      Some(NonEmpty(vec))
    }
  }

  /// Construct a non-empty vector from a `&[T]`.
  ///
  /// This function fails if the input slice is empty.
  pub fn from_slice(slice: &[T]) -> Option<Self> where T: Clone {
    if slice.is_empty() {
      None
    } else {
      Some(NonEmpty(slice.to_owned()))
    }
  }
}
