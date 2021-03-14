use std::ffi::CString;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size - we need to
    // write returned error to buffer,
    // therefore we need to know the required length of this buffer
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    // fill it with len spaces - (going to reuse the same allocation, as well as append 0 at the end):
    buffer.extend([b' '].iter().cycle().take(len as usize));
    // convert buffer to CString

    unsafe { CString::from_vec_unchecked(buffer) } //todo unsafe not either
}
