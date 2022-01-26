## Compile

Compile rust to a static library *ndarray_to_img.a* using `cargo build`

Compile the static C++ wrapper library to get
```
g++ -c src/wrapper.cpp -L ./target/debug/ -lndarray_to_img -static -o ndarray_to_img.o
```

Generate static library
```
ar rvs libndarray_to_img.a ndarray_to_img.o
```

Run the example
...


## Explanation

Create an RGB [image](https://docs.rs/image/0.23.14/image/type.RgbImage.html)
out of a 2D [ndarray](https://docs.rs/ndarray/0.15.4/ndarray/index.html) matrix.

Visualize sparse matrices.
Values >= 1 are represented by black cells and zeros by white cells.

For example, to generate the below 500x500 image
(in actuality a 501x501 image) out of a 10x10 matrix.

![test image matrix](./Figures/test_image_500x500.png)

```
let config = Config {
		verbosity: 0,
		annotate_image: true,
		draw_diagonal: true,
		draw_boundaries: true,
		scaling_factor: 50,
};

let mut matrix = Array2::<u8>::zeros((10, 10));
matrix[[0,1]] = 1;

// |----------|---|---|-----|----------|
// |          | 0 | 1 | ... | 9 or     |
// |          |   |   |     | j_max or |
// |          |   |   |     | x_max    |
// |----------|---|---|-----|----------|
// | 0        | 0 | 1 | ... | 0        |
// |----------|---|---|-----|----------|
// | 1        | 0 | 0 | ... | 0        |
// |----------|---|---|-----|----------|
// | .        | . | . | .   | .        |
// | .        | . | . |  .  | .        |
// | .        | . | . |   . | .        |
// |----------|---|---|-----|----------|
// | 9 or     |   |   |     |          |
// | i_max or | 0 | 0 | ... | 0        |
// | y_max    |   |   |     |          |
// |----------|---|---|-----|----------|


let scaled_matrix = scale_matrix(&matrix, &config);
let image_name = "image.png";
assert_eq!(generate_image(&scaled_matrix, &config, image_name).unwrap(), ());
```
