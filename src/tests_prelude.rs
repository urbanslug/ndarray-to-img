#![cfg(test)]
use crate::types;

pub const CLEANUP_TESTS: bool = false;

pub static CONFIG: types::Config =  types::Config {
	verbosity: 1,
	with_color: true,
	annotate_image: true,
	draw_diagonal: true,
	draw_boundaries: true,
	scaling_factor: 10,
};
