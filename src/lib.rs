use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::raw::c_char;
mod nitf;


// const FHDR: usize = 4;
// const FVER: usize = 5;
// const CLEVEL: usize = 2;
// const STYPE: usize = 4;
// const OSTAID: usize = 10;
// const FDT: usize = 14;
// const FTITLE: usize = 80;
// const FSCLAS: usize = 1;
// const FSCLSY: usize = 2;
// const FSCODE: usize = 11;
// const FSCTLH: usize = 2;
// const FSREL: usize = 20;
// const FSDCTP: usize = 2;
// const FSDCDT: usize = 8;
// const FSDCXM: usize = 4;
// const FSDG: usize = 1;
// const FSDGDT: usize = 8;
// const FSCLTX: usize = 43;
// const FSCATP: usize = 1;
// const FSCAUT: usize = 40;
// const FSCRSN: usize = 1;
// const FSSRDT: usize = 8;
// const FSCTLN: usize = 15;
// const FSCOP: usize = 5;
// const FSCPYS: usize = 5;
// const ENCRYP: usize = 1;
// const FBKGC: usize = 3;
// const ONAME: usize = 24;
// const OPHONE: usize = 18;
// const FL: usize = 12;
// const HL: usize = 6;
// const NUMI: usize = 3;
// const LISHN: usize = 6;
// const LIN: usize = 10;

// const HL_OFFSET: usize = FHDR
//     + FVER
//     + CLEVEL
//     + STYPE
//     + OSTAID
//     + FDT
//     + FTITLE
//     + FSCLAS
//     + FSCLSY
//     + FSCODE
//     + FSCTLH
//     + FSREL
//     + FSDCTP
//     + FSDCDT
//     + FSDCXM
//     + FSDG
//     + FSDGDT
//     + FSCLTX
//     + FSCATP
//     + FSCAUT
//     + FSCRSN
//     + FSSRDT
//     + FSCTLN
//     + FSCOP
//     + FSCPYS
//     + ENCRYP
//     + FBKGC
//     + ONAME
//     + OPHONE
//     + FL;
// const NUMI_OFFSET: usize = FHDR
//     + FVER
//     + CLEVEL
//     + STYPE
//     + OSTAID
//     + FDT
//     + FTITLE
//     + FSCLAS
//     + FSCLSY
//     + FSCODE
//     + FSCTLH
//     + FSREL
//     + FSDCTP
//     + FSDCDT
//     + FSDCXM
//     + FSDG
//     + FSDGDT
//     + FSCLTX
//     + FSCATP
//     + FSCAUT
//     + FSCRSN
//     + FSSRDT
//     + FSCTLN
//     + FSCOP
//     + FSCPYS
//     + ENCRYP
//     + FBKGC
//     + ONAME
//     + OPHONE
//     + FL
//     + HL;
//
#[no_mangle]
pub extern "C" fn extract_des(){}

#[no_mangle]
pub extern "C" fn extract_jp2(path_ptr: *const c_char, out_path_ptr: *const c_char) {
    let filepath_cstr = unsafe { CStr::from_ptr(path_ptr as *mut c_char) };
    let outpath_cstr = unsafe { CStr::from_ptr(out_path_ptr as *mut c_char) };
    let filepath = filepath_cstr
        .to_str()
        .expect("Failed to convert the file path to a string");
    let outpath = outpath_cstr
        .to_str()
        .expect("Failed to convert the output file path to a string");
    let mut file: File;
    match File::open(filepath) {
        Ok(f) => {
            file = f;
            println!("File opened Successfully: {:?}", filepath);
        }
        Err(error) => {
            println!("Failed to open the file: {}", error);
            return;
        }
    }

    let nitf = nitf::Nitf { ..Default::default() };
    let nitf = nitf.get_file_profile_name_and_version(&file);



    let _ = file.seek(SeekFrom::Start(HL_OFFSET as u64));
    let mut hl = [0u8; HL];
    let _ = file.read_exact(&mut hl);
    let hl_str = String::from_utf8_lossy(&hl);
    let hl_num: usize = hl_str
        .parse()
        .expect("Failed to coerce Header Length to usize");

    let _ = file.seek(SeekFrom::Start(NUMI_OFFSET as u64));
    let mut numi = [0u8; NUMI];
    let _ = file.read_exact(&mut numi);
    let numi_str = String::from_utf8_lossy(&numi);
    println!("NUMI: {}", numi_str);
    let num_images: usize = numi_str
        .parse()
        .expect("Image count string cannot be coerced to a number");
    write_jp2(&mut file, num_images, hl_num, outpath).expect("Failed to write jp2 files");
}

fn write_jp2(
    file: &mut File,
    num_images: usize,
    header_len: usize,
    outpath: &str,
) -> io::Result<()> {
    if num_images > 0 {
        let mut image_header_lens = std::vec::Vec::new();
        let mut image_data_lens = std::vec::Vec::new();
        for img in 0..num_images {
            let img_no = img + 1;
            let mut lish = [0u8; LISHN];
            let mut lin = [0u8; LIN];
            file.read_exact(&mut lish)?;
            file.read_exact(&mut lin)?;
            let lish_str = String::from_utf8_lossy(&lish);
            let lin_str = String::from_utf8_lossy(&lin);
            println!("IMAGE{} HEADER LEN: {}", img_no, lish_str);
            println!("IMAGE{} DATA LEN: {}", img_no, lin_str);
            let image_header_len: usize = lish_str
                .parse()
                .expect("Image Header Len String cannot be coerced to a number");
            let image_data_len: usize = lin_str
                .parse()
                .expect("Image Data Len String cannot be coerced to a number");
            image_header_lens.push(image_header_len);
            image_data_lens.push(image_data_len);
            // let mut img_seg = vec![0u8; image_header_len+image_data_len];
            // let _ = file.read_exact(&mut img_seg); //remove
        }
        let _ = file.seek(SeekFrom::Start(header_len as u64));
        for img in 0..num_images {
            let mut img_header = vec![0u8; image_header_lens[img]];
            let _ = file.read_exact(&mut img_header);
            let mut img_data = vec![0u8; image_data_lens[img]];
            let _ = file.read_exact(&mut img_data);
            let path = format!("{}{}.jp2", outpath, img);
            let mut out_file = File::create(path)?;
            let _ = out_file.write_all(&img_data);
        }
    }
    Ok(())
}
