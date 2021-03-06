//! Deprecated

// TODO: remove this module entirely
// Do not move before moving tests over as well

use image::{RgbaImage, Rgba};
use image::error::ImageResult;
use ndarray::{Array, Array2};
use num;

use crate::types;
use crate::constants::colors::*;


/// (Deprecated) Scale the 2 dimensional matrix by a scaling factor set in [Config](crate::Config).
///
/// Uses `floor(pos / scaling_factor)`.
// TODO: do we pay a cost for clone?
pub fn scale_matrix<T>(matrix: &Array2<Option<T>>, config: &types::Config) -> Array2<Option<T>>
where
	T: Clone
{
	if config.verbosity > 2 {
		eprintln!("[ndarray-to-img::scale_image]");
	}

	if config.scaling_factor == 1 {
		return matrix.clone();
	}

	let scaling_factor = config.scaling_factor as usize;

	let matrix_dimensions: &[usize] = matrix.shape();

	let i_max: usize = matrix_dimensions[0]; // rows
	let j_max: usize = matrix_dimensions[1]; // cols

	let scaled_i_max = i_max * scaling_factor; // scaled rows
	let scaled_j_max = j_max * scaling_factor; // scaled cols

	let mut scaled_matrix: Array2<Option<T>> = Array::from_elem((scaled_i_max, scaled_j_max), None);

	let scaling_factor = config.scaling_factor as f64;

	for i in 0..scaled_i_max {
		for j in 0..scaled_j_max {
			// TODO: should it be  B_i,j = A_ceil(i/5),ceil(j/5) ?
			let old_i = (i as f64/scaling_factor).floor() as usize;
			let old_j = (j as f64/scaling_factor).floor() as usize;

			scaled_matrix[[i, j]] = matrix[[old_i, old_j]].clone();
		}
	}

	scaled_matrix
}

fn max_and_min<T>(matrix: &Array2<Option<T>>) -> (T, T)
where
	T: num::Zero + Copy + std::cmp::PartialOrd
{
	// find a start value
	let mut max = num::zero();
	let mut min = num::zero();

	for opt_val in matrix.iter() {
		match opt_val {
			Some(x) => {
				max = *x;
				min = *x;
				break;
			},
			_ => {}
		}
	};

	// compare against all other values
	for opt_val in matrix.iter() {
		match opt_val {
			Some(val) => {
				if *val > max {
					max = *val
				}

				if *val < min {
					min = *val
				}
			},
			_ => {}
		}
	}

	(min, max)
}

/// (Deprecated) Generate the visualization of a 2D matrix from ndarray.
pub fn generate_image<T>(
	matrix: &Array2<Option<T>>,
	config: &types::Config,
	output_image_path: &str
) -> ImageResult<()>
where T: num::Zero + num::cast::ToPrimitive + Copy + std::cmp::PartialOrd
{
	if matrix.ndim() != 2 {
		panic!("[ndarray-to-img::generate_image] Expected a 2D matrix")
	}
	if config.verbosity > 2 {
		eprintln!("[ndarray-to-img::generate_image]");
	}

	if config.verbosity > 0 {
		eprintln!("Generating image {}", output_image_path);
		if config.verbosity > 1 {
			eprintln!("scaling factor: {}", config.scaling_factor);
		}
	}

	let (min, max) = max_and_min(matrix);

    // let (y_max, x_max) = matrix.dim();
	let matrix_dimensions: &[usize] = matrix.shape();

	let y_max = matrix_dimensions[0] as u32; // rows
	let x_max = matrix_dimensions[1] as u32; // cols

	// we add one to allow drawing the last vertical rows and cols
	let mut img = RgbaImage::new(x_max+1, y_max+1);

	let scaling_factor = config.scaling_factor as usize;

	// generate the image
	for x in 0..=x_max {
		for y in 0..=y_max {
			// Image annotations

            // use if...else
			if config.annotate_image {
				// Diagonals
				if config.draw_diagonal && x == y {
					img.put_pixel(x as u32, y as u32, RED);
					continue;
				}

				// vertical and horizontal separators
				if  config.draw_boundaries
                    && (x as usize % scaling_factor == 0)
                    || (y as usize % scaling_factor == 0)
                {
					img.put_pixel(x as u32, y as u32, BLUE);
					continue;
				}
			}

            if !config.annotate_image && (x == x_max || y == y_max) {
                continue;
            }

			// show pixel
			// we have to flip these to access the right cell in the matrix
			match matrix[[y as usize, x as usize]] {
				None => img.put_pixel(x as u32, y as u32, WHITE),
				Some(v) => {
					if v > num::zero() {
						let mut red = [255, 0, 0,  255];
						let value = v.to_f64().unwrap();
						let max_value = max.to_f64().unwrap();
						let m = u8::MAX as f64;
						let alpha_channel = ((value/max_value)*m).ceil() as u8;
						red[3] = alpha_channel;
						let red = Rgba::from(red);

						img.put_pixel(x as u32, y as u32, red);
					} else {
						let mut black = [0, 0, 0,  255];

						let value = v.to_f64().unwrap();
						let value = num::abs(value);

						let min_value = min.to_f64().unwrap();
						let min_value = num::abs(min_value);

						let m = u8::MAX as f64;
						let alpha_channel = ((value/min_value)*m).ceil() as u8;
						black[3] = alpha_channel;
						let black = Rgba::from(black);

						img.put_pixel(x as u32, y as u32, black);
					}
				}
			}
		}
	}

	// save it
	img.save(output_image_path)
}

#[cfg(test)]
mod tests {
	use super::*;
    use crate::tests_prelude;


	#[test]
    fn test_scale_matrix() {
		let matrix: Array2<Option<i32>> = Array::from_elem((10, 10), None);
		let scaled_matrix = scale_matrix(&matrix, &tests_prelude::CONFIG);

		assert_eq!(scaled_matrix.shape(), &[100, 100]);
    }

	#[test]
    fn test_scale_max() {
		let mut matrix: Array2<Option<i32>> = Array::from_elem((10, 10), None);

		matrix[[1,2]] = Some(1);
		matrix[[2,5]] = Some(7);
		matrix[[4,5]] = Some(10);
		matrix[[5,5]] = Some(5);
		matrix[[5,4]] = Some(-15);
		matrix[[8,9]] = Some(-190);

		let (min, max) = max_and_min(&matrix);

		assert_eq!(-190, min);
		assert_eq!(10, max);
    }

    #[test]
    fn test_generate_image() {
		let mut config = tests_prelude::CONFIG.clone();
		config.scaling_factor = 50;

		let mut matrix: Array2<Option<i32>> = Array::from_elem((10, 10), None);

		matrix[[1,2]] = Some(1);
		matrix[[2,5]] = Some(7);
		matrix[[4,5]] = Some(10);
		matrix[[5,5]] = Some(5);
		matrix[[5,4]] = Some(-15);
		matrix[[8,9]] = Some(-190);

		let scaled_matrix: Array2<Option<i32>> = scale_matrix(&matrix, &config);
		let image_name = "test_image.png";
        assert_eq!(generate_image(&scaled_matrix, &config, image_name).unwrap(), ());

		// clean up tests
		// let failed clean up result in error
		if tests_prelude::CLEANUP_TESTS {
			assert_eq!(std::fs::remove_file(image_name).unwrap(), ());
		}
    }
}
