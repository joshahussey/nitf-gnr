use std::io::{Read, Seek, SeekFrom};
use std::fs::File;

pub fn read_string_from_file(mut file: &File, offset: usize, length: usize) -> String {
    let mut file_slice_bytes = vec![0u8; length];
    file.seek(SeekFrom::Start(offset as u64))
        .expect("Failed to seek to offset.");
    let _ = file.read_exact(&mut file_slice_bytes);
    let file_slice_str = String::from_utf8_lossy(&file_slice_bytes).to_string();
    file_slice_str
}

pub fn read_int_from_file(mut file: &File, offset: usize, length: usize) -> usize {
    let mut file_slice_bytes = vec![0u8; length];
    file.seek(SeekFrom::Start(offset as u64))
        .expect("Failed to seek to offset.");
    let _ = file.read_exact(&mut file_slice_bytes);
    let file_slice_str = String::from_utf8_lossy(&file_slice_bytes);
    match file_slice_str.parse::<usize>() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Error: {}:\n String: {}", e, file_slice_str);
            panic!("File Slice String cannot be coerced to a number.");
        }
    }
}

pub fn read_string_from_bytes(data: &Vec<u8>, offset: usize, length: usize) -> String {
    let end = std::cmp::min(offset + length, data.len());
    let file_slice_bytes = &data[offset..end];
    String::from_utf8_lossy(file_slice_bytes).to_string()
}

pub fn read_int_from_bytes(data: &Vec<u8>, offset: usize, length: usize) -> usize {
    let end = std::cmp::min(offset + length, data.len());
    let file_slice_bytes = &data[offset..end];
    let file_slice_str = String::from_utf8_lossy(file_slice_bytes);
    file_slice_str
        .parse::<usize>()
        .expect("File Slice String cannot be coerced to a number.")
}
