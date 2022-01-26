use libc::{c_int, __u8, __u32, size_t};
use std::slice;
use ndarray::{Array2, Array};

pub mod rusty;
pub use rusty::*;

#[repr(C)]
#[derive(Debug)]
pub struct Position {
		pub x: __u32,
    pub y: __u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Color {
		pub red: __u8,
    pub green: __u8,
		pub blue: __u8,
		pub alpha: __u8
}

#[repr(C)]
#[derive(Debug)]
pub struct Cell {
		pub position: Position,
    pub value: c_int,
    pub color: Color,
}


#[no_mangle]
pub extern fn read_cells(
		data : *const Cell,
		length : size_t,
		nrow: size_t,
		ncol: size_t
) {
		let vec = unsafe{slice::from_raw_parts(data, length)};

    for x in vec {
				println !("{:?}", x);
		}

		call_rust(vec, nrow, ncol);
}

fn call_rust(data: &[Cell], nrow: usize, ncol: usize) {
		let config =  rusty::Config {
				verbosity: 1,
				with_color: true,
				annotate_image: true,
				draw_diagonal: true,
				draw_boundaries: true,
				scaling_factor: 10,
		};


		let dim = (nrow as usize, ncol as usize);
    let x = ndarray::Dim(dim);
    let mut matrix: Array2<Option<i32>> = Array::from_elem(x, None);

		for d in data {
				let x = d.position.x as usize;
				let y = d.position.y  as usize;
				matrix[[x, y]] = Some(d.value);
		}

		let scaled_matrix = rusty::scale_matrix(&matrix, &config);
    let image_name = "all.png";
    rusty::generate_image(&scaled_matrix, &config, &image_name).unwrap();
}
