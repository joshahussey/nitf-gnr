use nitf_gnr::modify::core;
mod helpers;

#[test]
fn get_version() {
    let file =
        std::fs::File::open("tests/nitf/Japan_1_Uncompressed.ntf").expect("Failed to open file");
    let (fhdr, ver) = core::get_version(&file);
    println!("File Header: {}", fhdr);
    println!("Version: {}", ver);
    let str = fhdr + &ver;
    assert_eq!(str, "NITF02.10");
}

#[test]
fn copy_des() {
    let mut input_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("tests/nitf/copyDes.ntf")
        .expect("Failed to open file");
    let mut output_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("tests/out/copyDes.ntf")
        .expect("Failed to open file");
    println!("Opened files");
    let num_des_pre = core::get_num_des(&output_file);
    println!("Number of Data Extensions: {}", num_des_pre);
    let num_des_add = core::get_num_des(&input_file);
    println!("Number of Data Extensions to add: {}", num_des_add);
    let valid_num_des = num_des_pre + num_des_add;
    core::copy_des_segmants(&mut input_file, &mut output_file);
    let num_des_post = core::get_num_des(&output_file);
    assert_eq!(num_des_post, valid_num_des);
}

#[test]
fn extract_all_jp2() {
    let input_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("tests/nitf/Japan_1_Uncompressed.ntf")
        .expect("Failed to open file");
    core::extract_jp2(&input_file, "tests/out/extract_all_jp2");
}

#[test]
fn extract_jp2_index() {
    let input_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("tests/nitf/Japan_1_Uncompressed.ntf")
        .expect("Failed to open file");
    let check_bytes = helpers::calculate_bytes_crc32(&core::extract_jp2_index(&input_file, 0).unwrap());
    let check = helpers::calculate_file_crc32("tests/out/extract_all_jp20.jp2").unwrap();
    assert_eq!(check_bytes, check);
}

