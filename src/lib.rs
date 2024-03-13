//! # RustTI
//!
//! A Technical Indicators library for Rust
//!
//! Each module is split into two submodules.
//!
//! A `single` submodule that is used to calculate the indicator once, using the entire price slice that is passed in.
//!
//! A `bulk` submodule that is used to iterate over a slice of prices to calculate the indicator for a passed in period.
//!
//! Many of the functions accept parameters that will allow the caller to move away from the technial
//! indicators from its default behaviour. For example, if a function normally uses the mean to calculate
//! the indicator, it can be told to use the median, or mode instead. More information is given in the
//! functions that allow this.

pub mod basic_indicators;
