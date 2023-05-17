use std::alloc::Layout;

#[no_mangle]
pub unsafe extern "C" fn apply(in_ptr: *mut u8, in_size: usize, out_ptr: &mut usize, out_size: &mut usize) {
    let input_string = String::from_raw_parts(in_ptr, in_size, in_size);
    let output_string = main(&input_string);
    let mut output_buffer = output_string.into_bytes();
    *out_size = output_buffer.len();
    *out_ptr = output_buffer.as_mut_ptr() as usize;
    std::mem::forget(output_buffer);
}

#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    std::alloc::alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    std::alloc::dealloc(ptr, layout);
}

fn main(input_string: &str) -> String {
    format!("Hello {}", input_string)
}