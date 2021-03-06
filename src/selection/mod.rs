//! The `selection` module provides implementations of the
//! `operator::SelectionOp` genetic operator.
//!
//! The provided `operator::SelectionOp`s are organized in sub-modules
//! named after the utilized selection method:
//! * `truncation`
//! * `ranking`
//! * `proportional`
//! * `tournament`

pub mod proportional;

pub mod ranking;

pub mod tournament;

pub mod truncation;
