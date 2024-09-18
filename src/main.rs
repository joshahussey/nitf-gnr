use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};




// variable input and output dirs
fn main() -> io::Result<()> {
    let nitf_file_path = "./testdata/input.nitf";
    let mut nitf_file = File::open(nitf_file_path).expect("Failed to open file: {nitf_file_path}.");

    let nitf = Nitf { ..Default::default() };
    let nitf = nitf.get_file_profile_name_and_version(&nitf_file);

    if nitf.file_profile_name != "NITF" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a valid NITF file",
        ));
    }

    let nitf = match nitf.file_version.as_str() {
        "02.10" => nitf.load_v02_10(),
        "02.00" => nitf.load_v02_00(),
        "01.10" =>
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Version 1.1 not yet implemented.",
        )),
        _ => 
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a supported NITF file version.",
        )),
    };

    let nitf = nitf.get_header_length_and_num_images(&nitf_file);
    let nitf = nitf.save_images(&nitf_file);

    dbg!(&nitf);

    Ok(())
}




