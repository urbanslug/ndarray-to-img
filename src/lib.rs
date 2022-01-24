/*!
Create an RGB [image](https://docs.rs/image/0.23.14/image/index.html)
out of a 2D [ndarray](https://docs.rs/ndarray/0.15.4/ndarray/index.html) matrix.

Meant to visualize sparse matrices.
Values >= 1 are represented by black cells and zeros by white cells.

For example, to generate a 100x100 image (in actuality a 101x101 image) out
of a 10x10 matrix.

# Example

```
use ndarray_to_img::{Config, scale_matrix, generate_image};
use ndarray::{Array2};

let config = Config {
    verbosity: 0,
    with_color: false,
    annotate_image: true,
    draw_diagonal: true,
    draw_boundaries: true,
    scaling_factor: 10,
};

let mut matrix = Array2::<u8>::zeros((10, 10));
matrix[[0,1]] = 1;

// |----------+---+---+-----+----------|
// |          | 0 | 1 | ... | 9 or     |
// |          |   |   |     | j_max or |
// |          |   |   |     | x_max    |
// |----------+---+---+-----+----------|
// | 0        | 0 | 1 | ... | 0        |
// |----------+---+---+-----+----------|
// | 1        | 0 | 0 | ... | 0        |
// |----------+---+---+-----+----------|
// | .        | . | . | .   | .        |
// | .        | . | . |  .  | .        |
// | .        | . | . |   . | .        |
// |----------+---+---+-----+----------|
// | 9 or     |   |   |     |          |
// | i_max or | 0 | 0 | ... | 0        |
// | y_max    |   |   |     |          |
// |----------+---+---+-----+----------|

let scaled_matrix = scale_matrix(&matrix, &config);
let image_name = "image.png";
assert_eq!(generate_image(&scaled_matrix, &config, image_name).unwrap(), ());

// clean up i.e. delete image
assert_eq!(std::fs::remove_file(image_name).unwrap(), ());
```
*/

use image::{RgbaImage, Rgba};
use image::error::ImageResult;
use ndarray::{Array2};
use num;

/// Configuration for the output image and library.
#[derive(Clone)]
pub struct Config {
		pub verbosity: u8,
		pub with_color: bool,
		pub annotate_image: bool,
		pub draw_diagonal: bool,
		pub draw_boundaries: bool, // draw row and column boundaries?
		pub scaling_factor: u8,
}

// Colors
const BLACK: Rgba<u8>  = Rgba([0, 0, 0, 255]);
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const RED: Rgba<u8> = Rgba([255, 0, 0,  125]);
const _GREEN: Rgba<u8> = Rgba([0, 255, 0,  255]);
const BLUE: Rgba<u8> = Rgba([0, 0, 255,  255]);

/// Scale the 2 dimensional matrix by a scaling factor set in [Config](self::Config).
///
/// Uses `floor(pos / scaling_factor)`.
// TODO: do we pay a cost for clone?
pub fn scale_matrix<T>(matrix: &Array2<T>, config: &Config) -> Array2<T>
where
		T: num::Unsigned + Clone
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

		let mut scaled_matrix = Array2::<T>::zeros((scaled_i_max, scaled_j_max));

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

pub fn max<T>(matrix: &Array2<T>) -> T
where
		T: num::Unsigned + Copy + std::cmp::PartialOrd
{
		let mut matrix_iter = matrix.iter();
		let mut max = *matrix_iter.next().unwrap();

		for val in matrix_iter {
				if *val > max {
						max = *val;
				}
		}
		max
}

/// Generate the visualization of a 2D matrix from ndarray.
pub fn generate_image<T>(
		matrix: &Array2<T>,
		config: &Config,
		output_image_path: &str
) -> ImageResult<()>
where T: num::Unsigned + num::cast::ToPrimitive + Copy + std::cmp::PartialOrd
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

		let mut max_value = num::zero();
		if config.with_color {
				max_value = max(matrix);
		}

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

						if config.annotate_image {
								// Diagonals
								if config.draw_diagonal && x == y {
										img.put_pixel(x as u32, y as u32, RED);
										continue;
								}

								// vertical and horizontal separators
								if  config.draw_boundaries && (x as usize % scaling_factor == 0) || (y as usize % scaling_factor == 0) {
										img.put_pixel(x as u32, y as u32, BLUE);
										continue;
								}
						}

						// show pixel
						// we have to flip these to access the right cell in the matrix
						if matrix[[y as usize, x as usize]] != num::zero() {

								if config.with_color {
										let mut red = [255, 0, 0,  255];
										let value = matrix[[y as usize, x as usize]];
										let value = value.to_f64().unwrap();
										let max_value = max_value.to_f64().unwrap();
										let m = u8::MAX as f64;
										let alpha_channel = ((value/max_value)*m).ceil() as u8;
										red[3] = alpha_channel;
										let red = Rgba::from(red);

										img.put_pixel(x as u32, y as u32, red);
								} else {
										img.put_pixel(x as u32, y as u32, BLACK);
								}
						} else {
								img.put_pixel(x as u32, y as u32, WHITE);
						}
				}
		}

		// save it
		img.save(output_image_path)
}

#[cfg(test)]
mod tests {
		use super::*;

		mod tests_config {
				pub const CLEANUP_TESTS: bool = true;

				pub static CONFIG: crate::Config =  crate::Config {
						verbosity: 1,
						with_color: true,
						annotate_image: true,
						draw_diagonal: true,
						draw_boundaries: true,
						scaling_factor: 10,
				};
		}

		#[test]
    fn test_scale_matrix() {
        let matrix = Array2::<u8>::zeros((10, 10));
				let scaled_matrix = scale_matrix(&matrix, &tests_config::CONFIG);

				assert_eq!(scaled_matrix.shape(), &[100, 100]);
    }

		#[test]
    fn test_scale_max() {
        let mut matrix = Array2::<u8>::zeros((10, 10));
				matrix[[1,2]] = 1;
				matrix[[4,5]] = 10;
				matrix[[2,5]] = 7;
				matrix[[5,5]] = 5;
				let max_value = max(&matrix);

				assert_eq!(10, max_value);
    }

    #[test]
    fn test_generate_image() {
				let mut config = tests_config::CONFIG.clone();
				config.scaling_factor = 50;

        let mut matrix = Array2::<u8>::zeros((10, 10));
				matrix[[1,2]] = 1;
				matrix[[4,5]] = 10;
				matrix[[2,5]] = 7;
				matrix[[5,5]] = 5;
				let scaled_matrix = scale_matrix(&matrix, &config);
				let image_name = "test_image.png";
        assert_eq!(generate_image(&scaled_matrix, &config, image_name).unwrap(), ());

				// clean up tests
				// let failed clean up result in error
				if tests_config::CLEANUP_TESTS {
						assert_eq!(std::fs::remove_file(image_name).unwrap(), ());
				}
    }
}
