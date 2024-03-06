//! # RustTI
//!
//! A Technical Indicators library for Rust
//!
//! Each module is split into two submodules.
//!
//! A `single` submodule that is used to calculate the indicator once, using the entire price slice that is passed in.
//!
//! A `bulk` submodule that is used to iterate over a slice of prices to calculate the indicator for a passed in period.

pub mod basic_indicators;
