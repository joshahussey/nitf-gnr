use std::ffi::CStr;
use std::fs::File;
use std::os::raw::c_char;
use crate::modify::parser::nitf21::{NitfHeader21 as N, NitfHeader21::*};
use crate::modify::parser::file_ops::read_int_from_file;
use super::core;

#[no_mangle]
pub extern "C" fn extract_jp2(path_ptr: *const c_char, out_path_ptr: *const c_char) {
    //String Manipulation and File Handling
    let outpath_cstr = unsafe { CStr::from_ptr(out_path_ptr as *mut c_char) };
    let outpath = outpath_cstr
        .to_str()
        .expect("Failed to convert the output file path to a string");
    let file = open_file_from_cstr(path_ptr);
    core::extract_jp2(&file, outpath);
}

#[no_mangle]
pub extern "C" fn extract_des(path_ptr: *const c_char, out_path_ptr: *const c_char) {
    //String Manipulation and File Handling
    let outpath_cstr = unsafe { CStr::from_ptr(out_path_ptr as *mut c_char) };
    let outpath = outpath_cstr
        .to_str()
        .expect("Failed to convert the output file path to a string");
    let file = open_file_from_cstr(path_ptr);
    core::extract_des(&file, outpath);
}

#[no_mangle]
pub extern "C" fn get_version(path_ptr: *const c_char) {
    let file = open_file_from_cstr(path_ptr);
    let (fhdr, ver) = core::get_version(&file);
    println!("NITF Type: {}{}", fhdr, ver);
}

#[no_mangle]
pub extern "C" fn get_num_images_from_file(path_ptr: *const c_char) -> usize {
    let file = open_file_from_cstr(path_ptr);
    read_int_from_file(&file, N::get_offset(NUMI, None), N::get_value(NUMI))
}

#[no_mangle]
pub extern "C" fn get_num_graphics_from_file(path_ptr: *const c_char) -> usize {
    let file = open_file_from_cstr(path_ptr);
    read_int_from_file(&file, N::get_offset(NUMS, Some(&file)), N::get_value(NUMS))
}

#[no_mangle]
pub extern "C" fn get_num_text_files_from_file(path_ptr: *const c_char) -> usize {
    let file = open_file_from_cstr(path_ptr);
    read_int_from_file(&file, N::get_offset(NUMT, Some(&file)), N::get_value(NUMT))
}

#[no_mangle]
pub extern "C" fn get_num_des_from_file(path_ptr: *const c_char) -> usize {
    let file = open_file_from_cstr(path_ptr);
    read_int_from_file(&file, N::get_offset(NUMDES, Some(&file)), N::get_value(NUMDES))
}

#[no_mangle]
pub extern "C" fn get_num_res_from_file(path_ptr: *const c_char) -> usize {
    let file = open_file_from_cstr(path_ptr);
    read_int_from_file(&file, N::get_offset(NUMRES, Some(&file)), N::get_value(NUMRES))
}


fn open_file_from_cstr(path_ptr: *const c_char) -> File {
    let filepath_cstr = unsafe { CStr::from_ptr(path_ptr as *mut c_char) };
    let filepath = filepath_cstr
        .to_str()
        .expect("Failed to convert the file path to a string");
    File::open(filepath).expect("Failed to open the file")
}
