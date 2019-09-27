#[repr(C)]
pub struct Buffer {
    len: u64,
    data: *mut u8,
}

#[no_mangle]
pub extern "C" fn buffer_free(buf: Buffer) {
    let slice = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len as usize) };
    let slice = slice.as_mut_ptr();
    unsafe {
        Box::from_raw(slice);
    }
}

pub unsafe fn buffer_to_slice(buf: *const Buffer) -> &'static [u8] {
    std::slice::from_raw_parts((*buf).data, (*buf).len as usize)
}

pub fn vector_into_buffer(output: &mut Buffer, vec: Vec<u8>) {
    let mut buf = vec.into_boxed_slice();
    output.data = buf.as_mut_ptr();
    output.len = buf.len() as u64;
    std::mem::forget(buf);
}

pub unsafe fn cstring_to_str(input: *const libc::c_char) -> &'static str {
    &std::ffi::CStr::from_ptr(input)
        .to_str()
        .expect("convert a C string to rust &str should be ok")
}
