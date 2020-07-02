#![cfg_attr(windows, feature(windows_by_handle))]
#![forbid(unsafe_code)]

extern crate jwalk;

mod aggregate;
mod common;
mod crossdev;
mod inodefilter;

pub mod traverse;

pub use aggregate::aggregate;
pub use common::*;
pub(crate) use inodefilter::InodeFilter;
