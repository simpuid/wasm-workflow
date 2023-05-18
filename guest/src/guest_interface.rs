use std::alloc::Layout;

use crate::executor::Executor;

pub trait GuestInterface: Executor {
	fn apply(in_ptr: i32, in_size: i32, out_ptr: i32, out_size: i32) {
		unsafe {
			let input_string =
				String::from_raw_parts(in_ptr as *mut u8, in_size as usize, in_size as usize);
			let output_string = Self::execute(&input_string);
			let mut output_buffer = output_string.into_bytes();
			*(out_size as *mut i32) = output_buffer.len() as i32;
			*(out_ptr as *mut i32) = output_buffer.as_mut_ptr() as i32;
			std::mem::forget(output_buffer);
		}
	}

	fn alloc(size: i32) -> i32 {
		unsafe {
			let align = std::mem::align_of::<usize>();
			let layout = Layout::from_size_align_unchecked(size as usize, align);
			std::alloc::alloc(layout) as i32
		}
	}

	fn dealloc(ptr: i32, size: i32) {
		unsafe {
			let align = std::mem::align_of::<usize>();
			let layout = Layout::from_size_align_unchecked(size as usize, align);
			std::alloc::dealloc(ptr as *mut u8, layout);
		}
	}
}

impl<T: Executor> GuestInterface for T {}
