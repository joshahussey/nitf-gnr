use std::ffi::CStr;
use std::io::{Read, Seek, SeekFrom};
use std::os::raw::c_char;
use std::fs::File;
pub fn read_int_from_file(mut file: &File, offset: usize, length: usize) -> usize {
    let mut file_slice_bytes = vec![0u8; length];
    file.seek(SeekFrom::Start(offset as u64))
        .expect("Failed to seek to offset.");
    let _ = file.read_exact(&mut file_slice_bytes);
    let file_slice_str = String::from_utf8_lossy(&file_slice_bytes);
    file_slice_str
        .parse()
        .expect("File Slice String cannot be coerced to a number.")
}

pub fn read_string_from_file(mut file: &File, offset: usize, length: usize) -> String {
    let mut file_slice_bytes = vec![0u8; length];
    file.seek(SeekFrom::Start(offset as u64))
        .expect("Failed to seek to offset.");
    let _ = file.read_exact(&mut file_slice_bytes);
    let file_slice_str = String::from_utf8_lossy(&file_slice_bytes).to_string();
    file_slice_str
}

pub fn open_file_from_cstr(path_ptr: *const c_char) -> File {
    let filepath_cstr = unsafe { CStr::from_ptr(path_ptr as *mut c_char) };
    let filepath = filepath_cstr
        .to_str()
        .expect("Failed to convert the file path to a string");
    File::open(filepath).expect("Failed to open the file")
}
