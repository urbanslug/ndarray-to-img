//! Plotting functions and trait
//!
//! Wrappers around matrices of `Option<T>` or `T`, where `T` is a number.
//!
//! For [OptMatrix](self::OptMatrix)
//!    - None is white
//!    - Some(<positive_number>) is a shade of red
//!    - Some(<negative_number>) is a shade of grey
//!
//! For [Matrix](self::Matrix)
//!    - 0 is white
//!    - positive_number is a shade of red
//!    - negative_number is a shade of grey


use num;
use image::error::ImageResult;
use ndarray::{Array, Array2};
use image::{RgbaImage, Rgba};

use crate::constants::colors::*;
use crate::types::Config;

// -------------
// Central trait
// -------------
/// The main trait of the library
pub trait Plottable<T> {
    fn plot(&self, config: &Config, output_image_path: &str)  -> ImageResult<()>;
    fn max_and_min(&self) -> (T, T);
    fn scale_matrix(&self, config: &Config) -> Self;
}


/// A wrapper around `Array2<Option<T>>`
#[derive(Debug)]
pub struct OptMatrix<T>
where T: num::Zero + num::cast::ToPrimitive + Copy  + Clone + std::cmp::PartialOrd + std::cmp::Ord
{
    pub matrix: Array2<Option<T>>
}

impl<T> Plottable<T> for OptMatrix<T>
where T: num::Zero + num::cast::ToPrimitive + Clone + Copy + std::cmp::PartialOrd + std::cmp::Ord
{
    fn plot(&self, config: &Config, output_image_path: &str) -> ImageResult<()> {
        let matrix = &self.matrix;

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

	    let (min, max) = self.max_and_min();

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

    fn max_and_min(&self) -> (T, T) {
        let matrix = &self.matrix;

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

    fn scale_matrix(&self, config: &Config) -> OptMatrix<T> {
        let matrix = &self.matrix;

	    if config.verbosity > 2 {
		    eprintln!("[ndarray-to-img::scale_image]");
	    }

	    if config.scaling_factor == 1 {
		    return OptMatrix{ matrix: matrix.clone() };
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

        OptMatrix{ matrix: scaled_matrix }
    }
}


/// A wrapper around `Array2<T>`
#[derive(Debug)]
pub struct Matrix<T>
where T: num::Zero + num::cast::ToPrimitive + Copy  + Clone + std::cmp::PartialOrd + std::cmp::Ord
{
    pub matrix: Array2<T>
}

impl<T> Plottable<T> for Matrix<T>
where T: num::Zero + num::cast::ToPrimitive + Clone + Copy + std::cmp::PartialOrd + std::cmp::Ord
{
    fn plot(&self, config: &Config, output_image_path: &str) -> ImageResult<()> {
        let matrix = &self.matrix;

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

	    let (min, max) = self.max_and_min();

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
                let value_at_cell = matrix[[y as usize, x as usize]];
                match value_at_cell.cmp(&num::zero()) {
                    std::cmp::Ordering::Equal => { img.put_pixel(x as u32, y as u32, WHITE); }
                    std::cmp::Ordering::Less => {
                        let mut black = [0, 0, 0,  255];

						let value = value_at_cell.to_f64().unwrap();
						let value = num::abs(value);

						let min_value = min.to_f64().unwrap();
						let min_value = num::abs(min_value);

						let m = u8::MAX as f64;
						let alpha_channel = ((value/min_value)*m).ceil() as u8;
						black[3] = alpha_channel;
						let black = Rgba::from(black);

						img.put_pixel(x as u32, y as u32, black);
                    },
                    std::cmp::Ordering::Greater => {
                        let mut red = [255, 0, 0,  255];
						let value = value_at_cell.to_f64().unwrap();
						let max_value = max.to_f64().unwrap();
						let m = u8::MAX as f64;
						let alpha_channel = ((value/max_value)*m).ceil() as u8;
						red[3] = alpha_channel;
						let red = Rgba::from(red);

						img.put_pixel(x as u32, y as u32, red);
                    },
                }
		    }
	    }

	    // save it
	    img.save(output_image_path)
    }

    fn max_and_min(&self) -> (T, T) {
        let matrix = &self.matrix;

        // find a start value
	    let mut max = num::zero();
	    let mut min = num::zero();


	    // compare against all other values
	    for val in matrix.iter() {
            if *val > max {
				max = *val
			}

			if *val < min {
				min = *val
			}
	    }

	    (min, max)
    }

    fn scale_matrix(&self, config: &Config) -> Matrix<T> {
        let matrix = &self.matrix;

	    if config.verbosity > 2 {
		    eprintln!("[ndarray-to-img::scale_image]");
	    }

	    if config.scaling_factor == 1 {
		    return Matrix{ matrix: matrix.clone() };
	    }

	    let scaling_factor = config.scaling_factor as usize;

	    let matrix_dimensions: &[usize] = matrix.shape();

	    let i_max: usize = matrix_dimensions[0]; // rows
	    let j_max: usize = matrix_dimensions[1]; // cols

	    let scaled_i_max = i_max * scaling_factor; // scaled rows
	    let scaled_j_max = j_max * scaling_factor; // scaled cols

	    let mut scaled_matrix: Array2<T> = Array::from_elem((scaled_i_max, scaled_j_max), num::zero());

	    let scaling_factor = config.scaling_factor as f64;

	    for i in 0..scaled_i_max {
		    for j in 0..scaled_j_max {
			    // TODO: should it be  B_i,j = A_ceil(i/5),ceil(j/5) ?
			    let old_i = (i as f64/scaling_factor).floor() as usize;
			    let old_j = (j as f64/scaling_factor).floor() as usize;

			    scaled_matrix[[i, j]] = matrix[[old_i, old_j]].clone();
		    }
	    }

        Matrix{ matrix: scaled_matrix }
    }
}



#[cfg(test)]
mod tests {
	use super::*;
    use crate::tests_prelude;

    #[test]
    fn test_generate_image_opt() {
		let mut config = tests_prelude::CONFIG.clone();
		config.scaling_factor = 50;

		let mut matrix: Array2<Option<i32>> = Array::from_elem((10, 10), None);

		matrix[[1,2]] = Some(1);
		matrix[[2,5]] = Some(7);
		matrix[[4,5]] = Some(10);
		matrix[[5,5]] = Some(5);
		matrix[[5,4]] = Some(-15);
		matrix[[8,9]] = Some(-190);

        let matrix = OptMatrix {
            matrix,
        };

		let scaled_matrix: OptMatrix<i32> = matrix.scale_matrix(&config);
		let image_name = "test_opt_image.png";
        assert_eq!(scaled_matrix.plot(&config, image_name).unwrap(), ());

		// clean up tests
		// let failed clean up result in error
		if tests_prelude::CLEANUP_TESTS {
			assert_eq!(std::fs::remove_file(image_name).unwrap(), ());
		}
    }

    #[test]
    fn test_generate_image() {
		let mut config = tests_prelude::CONFIG.clone();
		config.scaling_factor = 50;

		let mut matrix: Array2<i32> = Array::from_elem((10, 10), 0);

		matrix[[1,2]] = 1;
		matrix[[2,5]] = 7;
		matrix[[4,5]] = 10;
		matrix[[5,5]] = 5;
		matrix[[5,4]] = -15;
		matrix[[8,9]] = -190;

        let matrix = Matrix {
            matrix,
        };

		let scaled_matrix: Matrix<i32> = matrix.scale_matrix(&config);
		let image_name = "test_image.png";
        assert_eq!(scaled_matrix.plot(&config, image_name).unwrap(), ());

		// clean up tests
		// let failed clean up result in error
		if tests_prelude::CLEANUP_TESTS {
			assert_eq!(std::fs::remove_file(image_name).unwrap(), ());
		}
    }
}
