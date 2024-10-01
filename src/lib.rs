pub mod parser;
use std::ffi::CStr;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::raw::c_char;
use parser::nitf21::{NitfHeader21 as N, NitfHeader21::*};
use parser::file_ops::{read_int_from_file, open_file_from_cstr};

#[no_mangle]
pub extern "C" fn extract_jp2(path_ptr: *const c_char, out_path_ptr: *const c_char) {
    //String Manipulation and File Handling
    let outpath_cstr = unsafe { CStr::from_ptr(out_path_ptr as *mut c_char) };
    let outpath = outpath_cstr
        .to_str()
        .expect("Failed to convert the output file path to a string");
    let mut file = open_file_from_cstr(path_ptr);
    //Read the NITF file
    let nitf = parser::nitf21::Nitf {
        ..Default::default()
    };
    let nitf = nitf.get_file_profile_name_and_version(&file);
    if nitf.file_profile_name != "NITF" {
        eprint!("Not a valid NITF file");
        return;
    }
    for i in 0..get_num_images(&file) {
        let img_data_offset = N::get_image_data_field_offset(Some(&file), i as u64);
        let img_length = read_int_from_file(&file, img_data_offset, N::get_value(LINNN));
        let _ = file.seek(SeekFrom::Start(img_data_offset as u64));
        let mut img_data = vec![0u8; img_length];
        let _ = file.read_exact(&mut img_data);
        let path = format!("{}{}.jp2", outpath, i);
        let mut out_file = File::create(path).expect("Failed to create output file");
        let _ = out_file.write_all(&img_data);
    }
}

#[no_mangle]
pub extern "C" fn extract_des(path_ptr: *const c_char, out_path_ptr: *const c_char) {
    //String Manipulation and File Handling
    let outpath_cstr = unsafe { CStr::from_ptr(out_path_ptr as *mut c_char) };
    let outpath = outpath_cstr
        .to_str()
        .expect("Failed to convert the output file path to a string");
    let mut file = open_file_from_cstr(path_ptr);
    let num_des = read_int_from_file(&file, N::get_offset(NUMDES, Some(&file)), N::get_value(NUMDES));
    for i in 0..num_des {
        let des_header_offset = N::get_des_header_field_offset(Some(&file), i as u64);
        let des_header_length = read_int_from_file(&file, des_header_offset, N::get_value(LDNNN));
        let des_data_offset = N::get_des_data_field_offset(Some(&file), i as u64);
        let des_data_length = read_int_from_file(&file, des_data_offset, N::get_value(LDNNN));
        let _ = file.seek(SeekFrom::Start(des_header_offset as u64));
        let mut des_header = vec![0u8; des_header_length];
        let _ = file.read_exact(&mut des_header);
        let _ = file.seek(SeekFrom::Start(des_data_offset as u64));
        let mut des_data = vec![0u8; des_data_length];
        let _ = file.read_exact(&mut des_data);
        let des = format!("{}{}.des", outpath, i);
        let mut des_file = File::create(des).expect("Failed to create output file");
        let _ = des_file.write_all(&des_header);
        let _ = des_file.write_all(&des_data);
    }
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


//Private functions
fn get_num_images(file: &File) -> usize {
    read_int_from_file(file, N::get_offset(NUMI, None), N::get_value(NUMI))
}
