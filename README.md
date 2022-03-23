# ndarray to img
Visualize sparse matrices.

Create an RGB [image](https://docs.rs/image/0.23.14/image/type.RgbImage.html)
out of a 2D [ndarray](https://docs.rs/ndarray/0.15.4/ndarray/index.html) matrix.


## Compile
### C++
```
make
```

### Rust
```
cargo build --release
```

## Documentation
```
cargo doc --open
```

**Example Result:**

```rust
let config = Config {
		verbosity: 0,
		with_color: false,
		annotate_image: true,
		draw_diagonal: true,
		draw_boundaries: true,
		scaling_factor: 50,
};

let mut matrix = Array2::<u8>::zeros((10, 10));
matrix[[0,1]] = 1;

// Make a plottable version of the 2 dimensional array
let matrix = plot::Matrix { matrix }

// Scale it
let scaled_matrix: plot::Matrix<u8> = matrix.scale_matrix(&config);

// Plot
let image_name = "test_non_opt_image.png";
assert_eq!(scaled_matrix.plot(&config, image_name).unwrap(), ());

```

![test image matrix](./Figures/test_image_500x500.png)
