use crate::modify::parser;
use crate::modify::parser::file_ops::{read_int_from_file, read_int_from_bytes, read_string_from_file};
use crate::modify::parser::nitf21::{NitfHeader21 as N, NitfHeader21::*};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub fn get_version(file: &File) -> (String, String) {
    let fhdr = read_string_from_file(file, N::get_offset(FHDR, None), N::get_value(FHDR));
    let ver = read_string_from_file(file, N::get_offset(FVER, None), N::get_value(FVER));
    (fhdr, ver)
}

pub fn extract_jp2(mut file: &File, outpath: &str) {
    let nitf = parser::nitf21::Nitf {
        ..Default::default()
    };
    let nitf = nitf.get_file_profile_name_and_version(file);
    if nitf.file_profile_name != "NITF" {
        eprint!("Not a valid NITF file");
        return;
    }
    for i in 0..get_num_images(file) {
        let img_data_offset = N::get_image_data_field_offset(Some(file), i as u64);
        let img_length = read_int_from_file(file, img_data_offset, N::get_value(LINNN));
        let _ = file.seek(SeekFrom::Start(img_data_offset as u64));
        let mut img_data = vec![0u8; img_length];
        let _ = file.read_exact(&mut img_data);
        let path = format!("{}{}.jp2", outpath, i);
        let mut out_file = File::create(path).expect("Failed to create output file");
        let _ = out_file.write_all(&img_data);
    }
}

pub fn extract_jp2_index(mut file: &File, i: usize) -> Option<Vec<u8>> {
    if i >= get_num_images(file) {
        eprint!("Index out of bounds");
        return None;
    }
    let img_data_offset = N::get_image_data_field_offset(Some(file), i as u64);
    let img_length = read_int_from_file(file, img_data_offset, N::get_value(LINNN));
    let _ = file.seek(SeekFrom::Start(img_data_offset as u64));
    let mut img_data = vec![0u8; img_length];
    let _ = file.read_exact(&mut img_data);
    Some(img_data)
}

pub fn extract_des_header_fields_index(mut file: &File, index: usize) -> Option<Vec<u8>> {
    if index >= get_num_des(file) {
        eprint!("Index out of bounds");
        return None;
    }
    println!("Extracting DES header fields index {}", index);
    let mut des_header_field_offset = N::get_offset(LDSHNNN, Some(file));
    for _i in 0..index {
        des_header_field_offset += N::get_value(LDSHNNN) + N::get_value(LDNNN);
    }
    let mut des_header = vec![0u8; N::get_value(LDSHNNN)+N::get_value(LDNNN)];
    let _ = file.seek(SeekFrom::Start(des_header_field_offset as u64));
    let _ = file.read_exact(&mut des_header);
    Some(des_header)
}

pub fn extract_des_index(mut file: &File, i: usize) -> Option<Vec<u8>> {
    if i >= get_num_des(file) {
        eprint!("Index out of bounds");
        return None;
    }
    println!("Extracting DES index {}", i);
    let des_header_offset = N::get_des_header_field_offset(Some(file), i as u64);
    let des_header_length = read_int_from_file(file, des_header_offset, N::get_value(LDSHNNN));
    let des_data_offset = N::get_des_data_field_offset(Some(file), i as u64);
    let des_length = read_int_from_file(file, des_data_offset, N::get_value(LDNNN));
    let _ = file.seek(SeekFrom::Start(des_header_offset as u64));
    let mut des_header = vec![0u8; des_header_length];
    let _ = file.read_exact(&mut des_header);
    let _ = file.seek(SeekFrom::Start(des_data_offset as u64));
    let mut des_data = vec![0u8; des_length];
    let _ = file.read_exact(&mut des_data);
    let mut des_all = vec![0u8; des_header_length + des_length];
    des_all[..des_header_length].copy_from_slice(&des_header);
    des_all[des_header_length..].copy_from_slice(&des_data);
    Some(des_all)
}

pub fn extract_des(mut file: &File, outpath: &str) {
    let num_des = get_num_des(file);
    for i in 0..num_des {
        let des_header_offset = N::get_des_header_field_offset(Some(file), i as u64);
        let des_header_length = read_int_from_file(file, des_header_offset, N::get_value(LDNNN));
        let des_data_offset = N::get_des_data_field_offset(Some(file), i as u64);
        let des_data_length = read_int_from_file(file, des_data_offset, N::get_value(LDNNN));
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

pub fn copy_des_segmants(input_file: &mut File, output_file: &mut File) {
    //This is a complicated process. Ensure that you modify the file from the end to the beggining
    //so that you do not alter any offsets that you are relying on.
    //
    //Get Input header field length and data length
    println!("Copying DES segments");
    let num_des = get_num_des(input_file);
    let des_field_start = N::get_offset(NUMDES, Some(input_file));
    let des_field_end = N::get_offset(NUMRES, Some(input_file));
    let des_start = N::get_des_segment_start(input_file);
    let des_end = N::get_reserved_extension_segment_start(input_file);
    //In lengths
    let des_header_field_length = des_field_end - des_field_start;
    let des_data_length = des_end - des_start;
    
    //Get Current output numdes, header length, and file length
    let output_num_des = get_num_des(output_file);
    println!("Output num des: {}", output_num_des);
    let output_hl = read_int_from_file(output_file, N::get_offset(HL, None), N::get_value(HL));
    let output_fl = read_int_from_file(output_file, N::get_offset(FL, None), N::get_value(FL));
    println!("Output HL: {}", output_hl);
    println!("Output FL: {}", output_fl);
    println!("Output File New Length: {}", output_fl + des_header_field_length + des_data_length);
    //Get output header field length and data length
    //let output_des_field_start = N::get_offset(NUMDES, Some(output_file));
    let output_des_field_end = N::get_offset(NUMRES, Some(output_file));
    //let output_des_start = N::get_des_segment_start(output_file);
    let output_des_end = N::get_reserved_extension_segment_start(output_file);
    //Out lengths 
    //let output_des_header_field_length = output_des_field_end - output_des_field_start;
    //let output_des_data_length = output_des_end - output_des_start;
    
    //Get outfile as buffer
    let out_buf = &mut Vec::new();
    println!("Made ouput buf");
    let _ = output_file.seek(SeekFrom::Start(0));
    let _ = output_file.read_to_end(out_buf);
    
    //Read DES Header from Input
    let mut des_header_field = vec![0u8; des_header_field_length];
    _ = input_file.seek(SeekFrom::Start(des_field_start as u64));
    _ = input_file.read_exact(&mut des_header_field);
    //Read DES Data from Input
    let mut des_data = vec![0u8; des_data_length];
    _ = input_file.seek(SeekFrom::Start(des_start as u64));
    _ = input_file.read_exact(&mut des_data);
    println!("Read DES data");
    
    //splice DES Header and Data into Output
    out_buf.splice(output_des_end..output_des_end, des_data.iter().cloned());
    out_buf.splice(output_des_field_end..output_des_field_end, des_header_field.iter().cloned());
    println!("Spliced DES data");

    //Change HL, FL, and NUMDES values
    let total_num_des = (num_des + output_num_des).to_string();
    println!("Total num des: {}", total_num_des);
    let mut des_buf = vec![48; N::get_value(NUMDES)];
    let total_hl = (output_hl + des_header_field_length).to_string();
    let mut hl_buf = vec![48; N::get_value(HL)];
    let total_fl = (output_fl + des_data_length + des_header_field_length).to_string();
    let mut fl_buf = vec![48; N::get_value(FL)];
    if total_num_des.as_bytes().len() > N::get_value(NUMDES) {
        panic!("NUMDES value too large");
    }
    if total_hl.as_bytes().len() > N::get_value(HL) {
        panic!("HL value too large");
    }
    if total_fl.as_bytes().len() > N::get_value(FL) {
        panic!("FL value too large");
    }
    des_buf[N::get_value(NUMDES)-total_num_des.len()..].copy_from_slice(total_num_des.as_bytes());
    hl_buf[N::get_value(HL)-total_hl.len()..].copy_from_slice(total_hl.as_bytes());
    fl_buf[N::get_value(FL)-total_fl.len()..].copy_from_slice(total_fl.as_bytes());
    let des_offset = N::get_offset(NUMDES, Some(output_file));
    out_buf[des_offset..des_offset + N::get_value(NUMDES)].copy_from_slice(&des_buf);
    let hl_offset = N::get_offset(HL, None);
    out_buf[hl_offset..hl_offset + N::get_value(HL)].copy_from_slice(&hl_buf);
    let fl_offset = N::get_offset(FL, None);
    out_buf[fl_offset..fl_offset + N::get_value(FL)].copy_from_slice(&fl_buf);
    println!("Changed HL, FL, and NUMDES values");
    println!("des_buf: {:?}", des_buf);
    println!("hl_buf: {:?}", hl_buf);
    println!("fl_buf: {:?}", fl_buf);
    let _ = output_file.seek(SeekFrom::Start(0));
    let _ = output_file.set_len(out_buf.len() as u64);
    let _ = output_file.write_all(out_buf);
    println!("Wrote to output file");
    output_file.flush().expect("Failed to flush test file");

}

pub fn add_des_bytes(nitf_bytes: &mut Vec<u8>, des_header_bytes: &Vec<u8>, des_bytes: &Vec<u8>) {
    //Change HL, FL, and NUMDES values
    println!("Getting old DES bytes");
    let mut hl_value = get_hl_value_bytes(nitf_bytes);
    println!("HL value: {}", hl_value);
    hl_value += des_header_bytes.len() + des_bytes.len();
    change_hl_value_bytes(nitf_bytes, hl_value);
    println!("HL value changed to {}", hl_value);
    let mut fl_value = get_fl_value_bytes(nitf_bytes);
    fl_value += des_header_bytes.len() + des_bytes.len();
    change_fl_value_bytes(nitf_bytes, fl_value);
    println!("FL value changed to {}", fl_value);
    let mut numdes = get_numdes_value_bytes(nitf_bytes);
    numdes+=1;
    change_numdes_value_bytes(nitf_bytes, numdes);
    println!("NUMDES value changed to {}", numdes);
    //Add DES header
    let des_header_offset = N::get_offset_bytes(NUMRES, Some(nitf_bytes));
    println!("DES header offset: {}", des_header_offset);
    nitf_bytes.splice(des_header_offset..des_header_offset+des_header_bytes.len(), des_header_bytes.iter().cloned());
    println!("DES header added");
    //Add DES data
    let des_data_end_offset = N::get_reserved_segment_start_bytes(nitf_bytes);
    println!("DES data end offset: {}", des_data_end_offset);
    nitf_bytes.splice(des_data_end_offset..des_data_end_offset+des_bytes.len(), des_bytes.iter().cloned());
    println!("DES data added");
}

//Helper and Utility functions
pub fn get_num_des(file: &File) -> usize {
    read_int_from_file(file, N::get_offset(NUMDES, Some(file)), N::get_value(NUMDES))
}


//Private functions
fn get_num_images(file: &File) -> usize {
    read_int_from_file(file, N::get_offset(NUMI, None), N::get_value(NUMI))
}

fn get_hl_value_bytes(bytes: &Vec<u8>) -> usize {
    println!("Getting HL value");
    read_int_from_bytes(bytes, N::get_offset_bytes(HL, None), N::get_value(HL))
}

fn get_fl_value_bytes(bytes: &Vec<u8>) -> usize {
    read_int_from_bytes(bytes, N::get_offset_bytes(FL, None), N::get_value(FL))
}

fn get_numdes_value_bytes(bytes: &Vec<u8>) -> usize {
    read_int_from_bytes(bytes, N::get_offset_bytes(NUMDES, None), N::get_value(NUMDES))
}

fn change_fl_value_bytes(bytes: &mut Vec<u8>, fl: usize) {
    let fl_offset = N::get_offset_bytes(FL, None);
    let fl_value = N::get_value(FL);
    let fl_str = fl.to_string();
    let fl_str_bytes = fl_str.as_bytes();
    let fl_str_len = fl_str_bytes.len();
    let fl_str_len = std::cmp::min(fl_str_len, fl_value);
    bytes[fl_offset..fl_offset+fl_str_len].copy_from_slice(&fl_str_bytes[..fl_str_len]);
}

fn change_hl_value_bytes(bytes: &mut Vec<u8>, hl: usize) {
    let hl_offset = N::get_offset_bytes(HL, None);
    let hl_value = N::get_value(HL);
    let hl_str = hl.to_string();
    let hl_str_bytes = hl_str.as_bytes();
    let hl_str_len = hl_str_bytes.len();
    let hl_str_len = std::cmp::min(hl_str_len, hl_value);
    bytes[hl_offset..hl_offset+hl_str_len].copy_from_slice(&hl_str_bytes[..hl_str_len]);
}
fn change_numdes_value_bytes(bytes: &mut Vec<u8>, numdes: usize) {
    let numdes_offset = N::get_offset_bytes(NUMDES, None);
    let numdes_value = N::get_value(NUMDES);
    let numdes_str = numdes.to_string();
    let numdes_str_bytes = numdes_str.as_bytes();
    let numdes_str_len = numdes_str_bytes.len();
    let numdes_str_len = std::cmp::min(numdes_str_len, numdes_value);
    bytes[numdes_offset..numdes_offset+numdes_str_len].copy_from_slice(&numdes_str_bytes[..numdes_str_len]);
}
