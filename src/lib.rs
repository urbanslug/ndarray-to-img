/*!
Create an RGB [image](https://docs.rs/image/0.23.14/image/index.html)
out of a 2D [ndarray](https://docs.rs/ndarray/0.15.4/ndarray/index.html) matrix.

Visualize sparse matrices.
Values >= 1 are represented by black cells and zeros by white cells.

For example, to generate a 100x100 image (in actuality a 101x101 image) out
of a 10x10 matrix.

# Example

```
use ndarray_to_img;
use ndarray_to_img::plot::{self, Plottable};
use ndarray::{Array, Array2};

fn example_fn() {
    // ------
    // Config
    // ------
    let mut config =  ndarray_to_img::Config {
	    verbosity: 1,
	    with_color: true,
	    annotate_image: true,
	    draw_diagonal: true,
	    draw_boundaries: true,
	    scaling_factor: 10,
    };

    config.scaling_factor = 50;

    // ------------
    // OptMatrix<T>
    // ------------

	let mut matrix: Array2<Option<i32>> = Array::from_elem((10, 10), None);

	matrix[[1,2]] = Some(1);
	matrix[[2,5]] = Some(7);
	matrix[[4,5]] = Some(10);
	matrix[[5,5]] = Some(5);
	matrix[[5,4]] = Some(-15);
	matrix[[8,9]] = Some(-190);

    let matrix = plot::OptMatrix {
        matrix,
    };

	let scaled_matrix: plot::OptMatrix<i32> = matrix.scale_matrix(&config);
	let image_file_path = "test_opt_image.png";

    assert_eq!(scaled_matrix.plot(&config, image_file_path).unwrap(), ());

    // ---------
    // Matrix<T>
    // ---------

    let mut matrix: Array2<i32> = Array::from_elem((10, 10), 0);

	matrix[[1,2]] = 1;
	matrix[[2,5]] = 7;
	matrix[[4,5]] = 10;
	matrix[[5,5]] = 5;
	matrix[[5,4]] = -15;
	matrix[[8,9]] = -190;

    let matrix = plot::Matrix {
        matrix,
    };

	let scaled_matrix: plot::Matrix<i32> = matrix.scale_matrix(&config);
	let image_name = "test_non_opt_image.png";
    assert_eq!(scaled_matrix.plot(&config, image_name).unwrap(), ());
}
```
 */


use libc::{c_int, __u8, __u32, size_t};
use std::slice;
use ndarray::{Array2, Array};


mod constants;
mod tests_prelude;
mod types;
pub mod plot;
mod rusty;
pub use rusty::*;
pub use types::Config;

/// For C++ FFI
#[repr(C)]
#[derive(Debug)]
pub struct Position {
		pub x: __u32,
    pub y: __u32,
}

/// For C++ FFI
#[repr(C)]
#[derive(Debug)]
pub struct Color {
		pub red: __u8,
    pub green: __u8,
		pub blue: __u8,
		pub alpha: __u8
}

/// For C++ FFI
#[repr(C)]
#[derive(Debug)]
pub struct Cell {
		pub position: Position,
    pub value: c_int,
    pub color: Color,
}


/// For C++ FFI
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
		let config =  types::Config {
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
