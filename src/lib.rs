//! Fontdue is a font parser, rasterizer, and layout tool.
//!
//! This is a no_std crate, but still requires the alloc crate.

#![cfg_attr(all(not(test)), no_std)]

extern crate alloc;

mod math;
mod platform;
mod raster;

pub use math::{FinalizedGeometry, Geometry, OutlineBuilder};
pub use raster::Raster;
