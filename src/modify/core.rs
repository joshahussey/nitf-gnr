use crate::modify::parser::file_ops::{
    read_int_from_bytes, read_int_from_file, read_string_from_file,
};
use crate::modify::parser::nitf21::Nitf;
use crate::modify::parser::nitf21::{NitfHeader21 as N, NitfHeader21::*};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub fn get_version(file: &File) -> (String, String) {
    let fhdr = read_string_from_file(file, N::get_offset(FHDR, None), N::get_value(FHDR));
    let ver = read_string_from_file(file, N::get_offset(FVER, None), N::get_value(FVER));
    (fhdr, ver)
}

pub fn extract_jp2(mut file: &File, outpath: &str) {
    for i in 0..get_num_images(file) {
        let img_data_offset = N::get_image_data_field_offset(Some(file), i as u64);
        let img_length = read_int_from_file(file, img_data_offset, N::get_value(LI));
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
    let img_length = read_int_from_file(file, img_data_offset, N::get_value(LI));
    let _ = file.seek(SeekFrom::Start(img_data_offset as u64));
    let mut img_data = vec![0u8; img_length];
    let _ = file.read_exact(&mut img_data);
    Some(img_data)
}

pub fn extract_des_header_fields_index(mut file: &File, index: usize) -> Option<Vec<u8>> {
    if index >= get_numdes(file) {
        eprint!("Index out of bounds");
        return None;
    }
    #[cfg(all(debug_assertions, not(test)))]
    println!("Extracting DES header fields index {}", index);
    let mut des_header_field_offset = N::get_offset(LDSH, Some(file));
    for _i in 0..index {
        des_header_field_offset += N::get_value(LDSH) + N::get_value(LD);
    }
    let mut des_header = vec![0u8; N::get_value(LDSH) + N::get_value(LD)];
    let _ = file.seek(SeekFrom::Start(des_header_field_offset as u64));
    let _ = file.read_exact(&mut des_header);
    Some(des_header)
}

pub fn extract_des_index(mut file: &File, i: usize) -> Option<Vec<u8>> {
    if i >= get_numdes(file) {
        eprint!("Index out of bounds");
        return None;
    }
    #[cfg(all(debug_assertions, not(test)))]
    println!("Extracting DES index {}", i);
    let des_header_offset = N::get_des_header_field_offset(Some(file), i as u64);
    let des_header_length = read_int_from_file(file, des_header_offset, N::get_value(LDSH));
    let des_data_offset = N::get_des_data_field_offset(Some(file), i as u64);
    let des_length = read_int_from_file(file, des_data_offset, N::get_value(LD));
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
    let num_des = get_numdes(file);
    for i in 0..num_des {
        let des_header_offset = N::get_des_header_field_offset(Some(file), i as u64);
        let des_header_length = read_int_from_file(file, des_header_offset, N::get_value(LD));
        let des_data_offset = N::get_des_data_field_offset(Some(file), i as u64);
        let des_data_length = read_int_from_file(file, des_data_offset, N::get_value(LD));
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

pub fn copy_des_segments(input_file: &mut File, output_file: &mut File) {
    //Retrieve offsets and lengths
    let input_nitf = Nitf::new(input_file);
    let input_num_des = get_numdes(input_file);
    let input_des_length = input_nitf.segments.reserved_extension_segments - input_nitf.segments.data_extension_segments;
    let input_des_header_length = input_nitf.get_data_extension_headers_length(input_file);

    let output_nitf = Nitf::new(output_file);
    let output_num_des = get_numdes(output_file);
    //End of DES segment is start of reserved extension segment, same for header
    let output_des_end = output_nitf.segments.reserved_extension_segments;
    let output_des_field_end = N::get_offset(NUMRES, Some(output_file));
    let output_hl = read_int_from_file(output_file, output_nitf.header.hl, N::get_value(HL));
    let output_fl = read_int_from_file(output_file, output_nitf.header.fl, N::get_value(FL));

    //Read input DES header
    let mut des_header = vec![0u8; input_des_header_length];
    _ = input_file.seek(SeekFrom::Start(input_nitf.header.ldsh as u64));
    _ = input_file.read_exact(&mut des_header);
    #[cfg(all(debug_assertions, not(test)))]
    {
    println!("Successfully read DES header");
    println!("Output Des End: {}", output_des_end);
    println!("Output Buf Length: {}", output_file.metadata().unwrap().len()); 
    }

    //Read input DES data
    let mut des_data = vec![0u8; input_des_length];
    _ = input_file.seek(SeekFrom::Start(input_nitf.segments.data_extension_segments as u64));
    _ = input_file.read_exact(&mut des_data);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Successfully read DES data");

    //Get Ouput file as buffer
    #[cfg(all(debug_assertions, not(test)))]
    println!("Making Output Buffer");
    let out_buf = &mut Vec::new();
    let _ = output_file.seek(SeekFrom::Start(0));
    let _ = output_file.read_to_end(out_buf);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Made Output Buf of length {}", out_buf.len());
    
    //splice DES Header and Data into Output
    #[cfg(all(debug_assertions, not(test)))]
    println!("Splicing DES data");
    //TODO: HERE IS THE ERROR
    out_buf.splice(output_des_end..output_des_end, des_data.iter().cloned());
    #[cfg(all(debug_assertions, not(test)))]
    println!("Spliced DES data, splicing DES header");
    out_buf.splice(
        output_des_field_end..output_des_field_end,
        des_header.iter().cloned(),
    );
    #[cfg(all(debug_assertions, not(test)))]
    println!("Spliced DES data");

    //Change HL, FL, and NUMDES values
    #[cfg(all(debug_assertions, not(test)))]
    println!("Changing HL, FL, and NUMDES values");
    let total_num_des = (input_num_des + output_num_des).to_string();
    #[cfg(all(debug_assertions, not(test)))]
    println!("Total Output Num Des: {}", total_num_des);
    let mut des_buf = vec![48; N::get_value(NUMDES)];
    let total_hl = (output_hl + input_des_header_length).to_string();
    let mut hl_buf = vec![48; N::get_value(HL)];
    let total_fl = (output_fl + input_des_length + input_des_header_length).to_string();
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
    des_buf[N::get_value(NUMDES) - total_num_des.len()..].copy_from_slice(total_num_des.as_bytes());
    hl_buf[N::get_value(HL) - total_hl.len()..].copy_from_slice(total_hl.as_bytes());
    fl_buf[N::get_value(FL) - total_fl.len()..].copy_from_slice(total_fl.as_bytes());
    N::change_field_value(NUMDES, output_file, out_buf, &des_buf);
    N::change_field_value(HL, output_file, out_buf, &hl_buf);
    N::change_field_value(FL, output_file, out_buf, &fl_buf);
    #[cfg(all(debug_assertions, not(test)))]
    {
        println!("Changed HL, FL, and NUMDES values");
        println!("des_buf: {:?}", des_buf);
        println!("hl_buf: {:?}", hl_buf);
        println!("fl_buf: {:?}", fl_buf);
    }
    let _ = output_file.seek(SeekFrom::Start(0));
    let _ = output_file.set_len(out_buf.len() as u64);
    let _ = output_file.write_all(out_buf);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Wrote to output file");
    output_file.flush().expect("Failed to flush test file");
    #[cfg(all(debug_assertions, not(test)))]
    {
        println!("New Values: ");
        println!("NUMDES: {}", get_numdes(output_file));
        println!("HL: {}", get_hl(output_file));
        println!("FL: {}", get_fl(output_file));
    }

}

pub fn copy_graphic_segments(input_file: &mut File, output_file: &mut File) {
    //Retrieve offsets and lengths
    let input_nitf = Nitf::new(input_file);
    let input_nums = get_nums(input_file);
    let input_graphic_length = input_nitf.segments.text_segments - input_nitf.segments.graphic_segments;
    let input_graphic_header_length = input_nitf.get_graphic_headers_length(input_file);

    let output_nitf = Nitf::new(output_file);
    let output_nums = get_nums(output_file);
    //End of graphic segment is start of text segment, header end is numx
    let output_graphic_end = output_nitf.segments.text_segments;
    let output_graphic_field_end = output_nitf.header.numx;
    let output_hl = get_hl(output_file);
    let output_fl = get_fl(output_file);

    //Read input DES header
    let mut graphic_header = vec![0u8; input_graphic_header_length];
    _ = input_file.seek(SeekFrom::Start(input_nitf.header.lssh as u64));
    _ = input_file.read_exact(&mut graphic_header);
    #[cfg(all(debug_assertions, not(test)))]
    {
    println!("Successfully read Graphic header");
    println!("Output Graphic End: {}", output_graphic_end);
    println!("Output Buf Length: {}", output_file.metadata().unwrap().len()); 
    }

    //Read input graphic data
    let mut graphic_data = vec![0u8; input_graphic_length];
    _ = input_file.seek(SeekFrom::Start(input_nitf.segments.graphic_segments as u64));
    _ = input_file.read_exact(&mut graphic_data);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Successfully read graphic data");

    //Get Ouput file as buffer
    #[cfg(all(debug_assertions, not(test)))]
    println!("Making Output Buffer");
    let out_buf = &mut Vec::new();
    let _ = output_file.seek(SeekFrom::Start(0));
    let _ = output_file.read_to_end(out_buf);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Made Output Buf of length {}", out_buf.len());
    
    //splice Graphic Header and Data into Output
    #[cfg(all(debug_assertions, not(test)))]
    println!("Splicing graphic data");
    //TODO: HERE IS THE ERROR
    out_buf.splice(output_graphic_end..output_graphic_end, graphic_data.iter().cloned());
    #[cfg(all(debug_assertions, not(test)))]
    println!("Spliced graphic data, splicing graphic header");
    out_buf.splice(
        output_graphic_field_end..output_graphic_field_end,
        graphic_header.iter().cloned(),
    );
    #[cfg(all(debug_assertions, not(test)))]
    println!("Spliced graphic data");

    //Change HL, FL, and NUMS values
    #[cfg(all(debug_assertions, not(test)))]
    println!("Changing HL, FL, and NUMS values");
    let total_nums = (input_nums + output_nums).to_string();
    #[cfg(all(debug_assertions, not(test)))]
    println!("Total Output Nums: {}", total_nums);
    let mut graphic_buf = vec![48; N::get_value(NUMS)];
    let total_hl = (output_hl + input_graphic_header_length).to_string();
    let mut hl_buf = vec![48; N::get_value(HL)];
    let total_fl = (output_fl + input_graphic_length + input_graphic_header_length).to_string();
    let mut fl_buf = vec![48; N::get_value(FL)];
    if total_nums.as_bytes().len() > N::get_value(NUMS) {
        panic!("NUMS value too large");
    }
    if total_hl.as_bytes().len() > N::get_value(HL) {
        panic!("HL value too large");
    }
    if total_fl.as_bytes().len() > N::get_value(FL) {
        panic!("FL value too large");
    }
    graphic_buf[N::get_value(NUMS) - total_nums.len()..].copy_from_slice(total_nums.as_bytes());
    hl_buf[N::get_value(HL) - total_hl.len()..].copy_from_slice(total_hl.as_bytes());
    fl_buf[N::get_value(FL) - total_fl.len()..].copy_from_slice(total_fl.as_bytes());
    N::change_field_value(NUMS, output_file, out_buf, &graphic_buf);
    N::change_field_value(HL, output_file, out_buf, &hl_buf);
    N::change_field_value(FL, output_file, out_buf, &fl_buf);
    #[cfg(all(debug_assertions, not(test)))]
    {
        println!("Changed HL, FL, and NUMS values");
        println!("graphic_buf: {:?}", graphic_buf);
        println!("hl_buf: {:?}", hl_buf);
        println!("fl_buf: {:?}", fl_buf);
    }
    let _ = output_file.seek(SeekFrom::Start(0));
    let _ = output_file.set_len(out_buf.len() as u64);
    let _ = output_file.write_all(out_buf);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Wrote to output file");
    output_file.flush().expect("Failed to flush test file");
    #[cfg(all(debug_assertions, not(test)))]
    {
        println!("New Values: ");
        println!("NUMS: {}", get_nums(output_file));
        println!("HL: {}", get_hl(output_file));
        println!("FL: {}", get_fl(output_file));
    }

}

pub fn copy_text_segments(input_file: &mut File, output_file: &mut File) {
    //Retrieve offsets and lengths
    let input_nitf = Nitf::new(input_file);
    let input_numt = get_numt(input_file);
    let input_t_length = input_nitf.segments.data_extension_segments - input_nitf.segments.text_segments;
    let input_t_header_length = input_nitf.get_text_headers_length(input_file);

    let output_nitf = Nitf::new(output_file);
    let output_numt = get_numt(output_file);
    //End of text segment is start of data extension segment, same for header
    let output_t_end = output_nitf.segments.data_extension_segments;
    let output_t_field_end = output_nitf.header.numdes;
    let output_hl = get_hl(output_file);
    let output_fl = get_fl(output_file);

    //Read input text header
    let mut t_header = vec![0u8; input_t_header_length];
    _ = input_file.seek(SeekFrom::Start(input_nitf.header.ltsh as u64));
    _ = input_file.read_exact(&mut t_header);
    #[cfg(all(debug_assertions, not(test)))]
    {
    println!("Successfully read text header");
    println!("Output text End: {}", output_t_end);
    println!("Output Buf Length: {}", output_file.metadata().unwrap().len()); 
    }

    //Read input text data
    let mut t_data = vec![0u8; input_t_length];
    _ = input_file.seek(SeekFrom::Start(input_nitf.segments.text_segments as u64));
    _ = input_file.read_exact(&mut t_data);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Successfully read text data");

    //Get Ouput file as buffer
    #[cfg(all(debug_assertions, not(test)))]
    println!("Making Output Buffer");
    let out_buf = &mut Vec::new();
    let _ = output_file.seek(SeekFrom::Start(0));
    let _ = output_file.read_to_end(out_buf);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Made Output Buf of length {}", out_buf.len());
    
    //splice DES Header and Data into Output
    #[cfg(all(debug_assertions, not(test)))]
    println!("Splicing text data");
    out_buf.splice(output_t_end..output_t_end, t_data.iter().cloned());
    #[cfg(all(debug_assertions, not(test)))]
    println!("Spliced text data, splicing text header");
    out_buf.splice(
        output_t_field_end..output_t_field_end,
        t_header.iter().cloned(),
    );
    #[cfg(all(debug_assertions, not(test)))]
    println!("Spliced text data");

    //Change HL, FL, and NUMT values
    #[cfg(all(debug_assertions, not(test)))]
    println!("Changing HL, FL, and NUMT values");
    let total_numt = (input_numt + output_numt).to_string();
    #[cfg(all(debug_assertions, not(test)))]
    println!("Total Output NUMT: {}", total_numt);
    let mut t_buf = vec![48; N::get_value(NUMT)];
    let total_hl = (output_hl + input_t_header_length).to_string();
    let mut hl_buf = vec![48; N::get_value(HL)];
    let total_fl = (output_fl + input_t_length + input_t_header_length).to_string();
    let mut fl_buf = vec![48; N::get_value(FL)];
    if total_numt.as_bytes().len() > N::get_value(NUMT) {
        panic!("NUMDES value too large");
    }
    if total_hl.as_bytes().len() > N::get_value(HL) {
        panic!("HL value too large");
    }
    if total_fl.as_bytes().len() > N::get_value(FL) {
        panic!("FL value too large");
    }
    t_buf[N::get_value(NUMT) - total_numt.len()..].copy_from_slice(total_numt.as_bytes());
    hl_buf[N::get_value(HL) - total_hl.len()..].copy_from_slice(total_hl.as_bytes());
    fl_buf[N::get_value(FL) - total_fl.len()..].copy_from_slice(total_fl.as_bytes());
    N::change_field_value(NUMT, output_file, out_buf, &t_buf);
    N::change_field_value(HL, output_file, out_buf, &hl_buf);
    N::change_field_value(FL, output_file, out_buf, &fl_buf);
    #[cfg(all(debug_assertions, not(test)))]
    {
        println!("Changed HL, FL, and NUMT values");
        println!("t_buf: {:?}", t_buf);
        println!("hl_buf: {:?}", hl_buf);
        println!("fl_buf: {:?}", fl_buf);
    }
    let _ = output_file.seek(SeekFrom::Start(0));
    let _ = output_file.set_len(out_buf.len() as u64);
    let _ = output_file.write_all(out_buf);
    #[cfg(all(debug_assertions, not(test)))]
    println!("Wrote to output file");
    output_file.flush().expect("Failed to flush test file");
    #[cfg(all(debug_assertions, not(test)))]
    {
        println!("New Values: ");
        println!("NUMT: {}", get_numt(output_file));
        println!("HL: {}", get_hl(output_file));
        println!("FL: {}", get_fl(output_file));
    }

}


//Helper and Utility functions
pub fn get_numdes(file: &File) -> usize {
    read_int_from_file(
        file,
        N::get_offset(NUMDES, Some(file)),
        N::get_value(NUMDES),
    )
}

pub fn get_nums(file: &File) -> usize {
    read_int_from_file(
        file,
        N::get_offset(NUMS, Some(file)),
        N::get_value(NUMS),
    )
}

pub fn get_numt(file: &File) -> usize {
    read_int_from_file(
        file,
        N::get_offset(NUMT, Some(file)),
        N::get_value(NUMT),
    )
}

pub fn get_hl(file: &File) -> usize {
    read_int_from_file(file, N::get_offset(HL, None), N::get_value(HL))
}

pub fn get_fl(file: &File) -> usize {
    read_int_from_file(file, N::get_offset(FL, None), N::get_value(FL))
}

//Private functions
fn get_num_images(file: &File) -> usize {
    read_int_from_file(file, N::get_offset(NUMI, None), N::get_value(NUMI))
}

fn get_hl_value_bytes(bytes: &Vec<u8>) -> usize {
    #[cfg(all(debug_assertions, not(test)))]
    println!("Getting HL value");
    read_int_from_bytes(bytes, N::get_offset_bytes(HL, None), N::get_value(HL))
}

fn get_fl_value_bytes(bytes: &Vec<u8>) -> usize {
    read_int_from_bytes(bytes, N::get_offset_bytes(FL, None), N::get_value(FL))
}

fn get_numdes_value_bytes(bytes: &Vec<u8>) -> usize {
    read_int_from_bytes(
        bytes,
        N::get_offset_bytes(NUMDES, None),
        N::get_value(NUMDES),
    )
}

fn change_fl_value_bytes(bytes: &mut Vec<u8>, fl: usize) {
    let fl_offset = N::get_offset_bytes(FL, None);
    let fl_value = N::get_value(FL);
    let fl_str = fl.to_string();
    let fl_str_bytes = fl_str.as_bytes();
    let fl_str_len = fl_str_bytes.len();
    let fl_str_len = std::cmp::min(fl_str_len, fl_value);
    bytes[fl_offset..fl_offset + fl_str_len].copy_from_slice(&fl_str_bytes[..fl_str_len]);
}

fn change_hl_value_bytes(bytes: &mut Vec<u8>, hl: usize) {
    let hl_offset = N::get_offset_bytes(HL, None);
    let hl_value = N::get_value(HL);
    let hl_str = hl.to_string();
    let hl_str_bytes = hl_str.as_bytes();
    let hl_str_len = hl_str_bytes.len();
    let hl_str_len = std::cmp::min(hl_str_len, hl_value);
    bytes[hl_offset..hl_offset + hl_str_len].copy_from_slice(&hl_str_bytes[..hl_str_len]);
}
fn change_numdes_value_bytes(bytes: &mut Vec<u8>, numdes: usize) {
    let numdes_offset = N::get_offset_bytes(NUMDES, None);
    let numdes_value = N::get_value(NUMDES);
    let numdes_str = numdes.to_string();
    let numdes_str_bytes = numdes_str.as_bytes();
    let numdes_str_len = numdes_str_bytes.len();
    let numdes_str_len = std::cmp::min(numdes_str_len, numdes_value);
    bytes[numdes_offset..numdes_offset + numdes_str_len]
        .copy_from_slice(&numdes_str_bytes[..numdes_str_len]);
}

// pub fn add_des_bytes(nitf_bytes: &mut Vec<u8>, des_header_bytes: &Vec<u8>, des_bytes: &Vec<u8>) {
//     //Change HL, FL, and NUMDES values
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("Getting old DES bytes");
//     let mut hl_value = get_hl_value_bytes(nitf_bytes);
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("HL value: {}", hl_value);
//     hl_value += des_header_bytes.len() + des_bytes.len();
//     change_hl_value_bytes(nitf_bytes, hl_value);
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("HL value changed to {}", hl_value);
//     let mut fl_value = get_fl_value_bytes(nitf_bytes);
//     fl_value += des_header_bytes.len() + des_bytes.len();
//     change_fl_value_bytes(nitf_bytes, fl_value);
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("FL value changed to {}", fl_value);
//     let mut numdes = get_numdes_value_bytes(nitf_bytes);
//     numdes += 1;
//     change_numdes_value_bytes(nitf_bytes, numdes);
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("NUMDES value changed to {}", numdes);
//     //Add DES header
//     let des_header_offset = N::get_offset_bytes(NUMRES, Some(nitf_bytes));
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("DES header offset: {}", des_header_offset);
//     nitf_bytes.splice(
//         des_header_offset..des_header_offset + des_header_bytes.len(),
//         des_header_bytes.iter().cloned(),
//     );
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("DES header added");
//     //Add DES data
//     let des_data_end_offset = N::get_reserved_segment_start_bytes(nitf_bytes);
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("DES data end offset: {}", des_data_end_offset);
//     nitf_bytes.splice(
//         des_data_end_offset..des_data_end_offset + des_bytes.len(),
//         des_bytes.iter().cloned(),
//     );
//     #[cfg(all(debug_assertions, not(test)))]
//     println!("DES data added");
// }

