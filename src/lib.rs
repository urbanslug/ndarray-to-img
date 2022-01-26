use libc::{c_float, c_int, __u8, __u32, size_t};
use std::slice;

#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}

#[no_mangle]
pub extern fn make_vec(input: i32) -> i32 {
    input * 2
}


#[no_mangle]
pub extern fn show_vector(data : *const c_float, size : size_t) {
		let vec = unsafe{std::slice::from_raw_parts(data, size)};



    for x in vec {
				println !("{}", x);
		}


}

#[no_mangle]
pub extern fn show_matrix(data : *const c_float, nrow : size_t, ncol : size_t) {
		// let vec = unsafe{std::slice::from_raw_parts(data, size)};

		let mut rows: Vec<&[f32]> = Vec::new();
    for i in 0..nrow as usize {
        rows.push(unsafe {
            slice::from_raw_parts(
                data.offset(i as isize * ncol as isize),
                ncol as usize
            )
        });
    }
    let matrix: &[&[f32]] = &rows[..];

    println!("{:#?}", matrix);
}

#[repr(C)]
#[derive(Debug)]
pub struct Cell {
    pub value: c_int,
    pub color: (__u8, __u8, __u8, __u8),
}

#[no_mangle]
pub extern fn read_cells(data : *const Cell, nrow : size_t, ncol : size_t) {
		// let vec = unsafe{std::slice::from_raw_parts(data, size)};

		let step = std::mem::size_of::<Cell>();

		eprintln!("rust {}", step);

		let mut rows: Vec<&[Cell]> = Vec::new();

    for i in 0..nrow as usize {
        rows.push(unsafe {
            slice::from_raw_parts(
                data.offset(i as isize * ncol as isize),
                ncol as usize
            )
        });
    }
    let matrix: &[&[Cell]] = &rows[..];

    println!("{:#?}", matrix);
}

#[repr(C)]
#[derive(Debug)]
pub struct Position {
		pub x: __u32,
    pub y: __u32,
}
#[repr(C)]
#[derive(Debug)]
pub struct CellP {
		pub position: (__u32, __u32),
    pub value: c_int,
    pub color: (__u8, __u8, __u8, __u8),
}


#[no_mangle]
pub extern fn read_cells_with_position(data : *const CellP, size : size_t) {
		// let vec = unsafe{std::slice::from_raw_parts(data, size)};

		let vec = unsafe{std::slice::from_raw_parts(data, size)};



    for x in vec {
				println !("{:?}", x);
		}

}
