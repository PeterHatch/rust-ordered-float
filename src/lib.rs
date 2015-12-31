#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! Wrappers for total order on Floats.

extern crate num;
extern crate unreachable;

mod ordered_float;
pub use ordered_float::OrderedFloat;

mod not_nan;
pub use not_nan::{NotNaN, FloatIsNaN};
