use libc::{c_int, __u8, __u32, size_t};
use std::slice;

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
		_nrow: size_t,
		_ncol: size_t
) {
		let vec = unsafe{slice::from_raw_parts(data, length)};

    for x in vec {
				println !("{:?}", x);
		}

}
