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
