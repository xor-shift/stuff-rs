#![allow(incomplete_features)]
#![allow(mixed_script_confusables)]
#![feature(adt_const_params)]
#![feature(allocator_api)]
#![feature(float_next_up_down)]
#![feature(generic_const_exprs)]
#![feature(inherent_associated_types)]
#![feature(iter_map_windows)]
#![feature(let_chains)]
#![feature(more_float_constants)]
#![feature(structural_match)]
#![feature(stmt_expr_attributes)]

pub mod integral;
pub mod numeric_limits;
pub mod qoi;
pub mod rng;
pub mod smallvec;
pub mod stl;

pub use integral::*;
pub use numeric_limits::*;
