use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};

const FHDR: usize = 4;
const FVER: usize = 5;
const CLEVEL: usize = 2;
const STYPE: usize = 4;
const OSTAID: usize = 10;
const FDT: usize = 14;
const FTITLE: usize = 80;
const FSCLAS: usize = 1;
const FSCLSY: usize = 2;
const FSCODE: usize = 11;
const FSCTLH: usize = 2;
const FSREL: usize = 20;
const FSDCTP: usize = 2;
const FSDCDT: usize = 8;
const FSDCXM: usize = 4;
const FSDG: usize = 1;
const FSDGDT: usize = 8;
const FSCLTX: usize = 43;
const FSCATP: usize = 1;
const FSCAUT: usize = 40;
const FSCRSN: usize = 1;
const FSSRDT: usize = 8;
const FSCTLN: usize = 15;
const FSCOP: usize = 5;
const FSCPYS: usize = 5;
const ENCRYP: usize = 1;
const FBKGC: usize = 3;
const ONAME: usize = 24;
const OPHONE: usize = 18;
const FL: usize = 12;
const HL: usize = 6;
const NUMI: usize = 3;
const LISHn: usize = 6;
const LIn: usize = 10;

fn main() -> io::Result<()> {
    let nitf_file = "input.nitf";
    let (jp2_offset, jp2_size) = read_nitf_headers(nitf_file)?;
    println!("JP2 Offset: {}, JP2 Size: {}", jp2_offset, jp2_size);
    Ok(())
}

fn read_nitf_headers(nitf_file: &str) -> io::Result<(u64, u64)> {
    let mut file = File::open(nitf_file)?;

    let mut file_type = [0u8; FHDR];
    let mut nitf_version = [0u8; FVER];
    let mut c_level = [0u8; CLEVEL];
    let mut s_type = [0u8; STYPE];
    let mut ostaid = [0u8; OSTAID];
    let mut fdt = [0u8; FDT];
    let mut ftitle = [0u8; FTITLE];
    let mut fsclas = [0u8; FSCLAS];
    let mut fsclsy = [0u8; FSCLSY];
    let mut fscode = [0u8; FSCODE];
    let mut fsctlh = [0u8; FSCTLH];
    let mut fsrel = [0u8; FSREL];
    let mut fsdctp = [0u8; FSDCTP];
    let mut fsdcdt = [0u8; FSDCDT];
    let mut fsdcxm = [0u8; FSDCXM];
    let mut fsdg = [0u8; FSDG];
    let mut fsdgdt = [0u8; FSDGDT];
    let mut fscltx = [0u8; FSCLTX];
    let mut fscatp = [0u8; FSCATP];
    let mut fscaut = [0u8; FSCAUT];
    let mut fscrsn = [0u8; FSCRSN];
    let mut fssrdt = [0u8; FSSRDT];
    let mut fsctln = [0u8; FSCTLN];
    let mut fscop = [0u8; FSCOP];
    let mut fscpys = [0u8; FSCPYS];
    let mut encryp = [0u8; ENCRYP];
    let mut fbkgc = [0u8; FBKGC];
    let mut oname = [0u8; ONAME];
    let mut ophone = [0u8; OPHONE];
    let mut fl = [0u8; FL];
    let mut hl = [0u8; HL];
    let mut numi = [0u8; NUMI];
    file.read_exact(&mut file_type)?;
    file.read_exact(&mut nitf_version)?;
    file.read_exact(&mut c_level)?;
    file.read_exact(&mut s_type)?;
    file.read_exact(&mut ostaid)?;
    file.read_exact(&mut fdt)?;
    file.read_exact(&mut ftitle)?;
    file.read_exact(&mut fsclas)?;
    file.read_exact(&mut fsclsy)?;
    file.read_exact(&mut fscode)?;
    file.read_exact(&mut fsctlh)?;
    file.read_exact(&mut fsrel)?;
    file.read_exact(&mut fsdctp)?;
    file.read_exact(&mut fsdcdt)?;
    file.read_exact(&mut fsdcxm)?;
    file.read_exact(&mut fsdg)?;
    file.read_exact(&mut fsdgdt)?;
    file.read_exact(&mut fscltx)?;
    file.read_exact(&mut fscatp)?;
    file.read_exact(&mut fscaut)?;
    file.read_exact(&mut fscrsn)?;
    file.read_exact(&mut fssrdt)?;
    file.read_exact(&mut fsctln)?;
    file.read_exact(&mut fscop)?;
    file.read_exact(&mut fscpys)?;
    file.read_exact(&mut encryp)?;
    file.read_exact(&mut fbkgc)?;
    file.read_exact(&mut oname)?;
    file.read_exact(&mut ophone)?;
    file.read_exact(&mut fl)?;
    file.read_exact(&mut hl)?;
    file.read_exact(&mut numi)?;
    let file_type_str = String::from_utf8_lossy(&file_type);
    let version_str = String::from_utf8_lossy(&nitf_version);
    let c_level_str = String::from_utf8_lossy(&c_level);
    let s_type_str = String::from_utf8_lossy(&s_type);
    let ostaid_str = String::from_utf8_lossy(&ostaid);
    let fdt_str = String::from_utf8_lossy(&fdt);
    let ftitle_str = String::from_utf8_lossy(&ftitle);
    let fsclas_str = String::from_utf8_lossy(&fsclas);
    let fsclsy_str = String::from_utf8_lossy(&fsclsy);
    let fscode_str = String::from_utf8_lossy(&fscode);
    let fsctlh_str = String::from_utf8_lossy(&fsctlh);
    let fsrel_str = String::from_utf8_lossy(&fsrel);
    let fsdctp_str = String::from_utf8_lossy(&fsdctp);
    let fsdcdt_str = String::from_utf8_lossy(&fsdcdt);
    let fsdcxm_str = String::from_utf8_lossy(&fsdcxm);
    let fsdg_str = String::from_utf8_lossy(&fsdg);
    let fsdgdt_str = String::from_utf8_lossy(&fsdgdt);
    let fscltx_str = String::from_utf8_lossy(&fscltx);
    let fscatp_str = String::from_utf8_lossy(&fscatp);
    let fscaut_str = String::from_utf8_lossy(&fscaut);
    let fscrsn_str = String::from_utf8_lossy(&fscrsn);
    let fssrdt_str = String::from_utf8_lossy(&fssrdt);
    let fsctln_str = String::from_utf8_lossy(&fsctln);
    let fscop_str = String::from_utf8_lossy(&fscop);
    let fscpys_str = String::from_utf8_lossy(&fscpys);
    let encryp_str = String::from_utf8_lossy(&encryp);
    let fbkgc_str = String::from_utf8_lossy(&fbkgc);
    let oname_str = String::from_utf8_lossy(&oname);
    let ophone_str = String::from_utf8_lossy(&ophone);
    let fl_str = String::from_utf8_lossy(&fl);
    let hl_str = String::from_utf8_lossy(&hl);
    let numi_str = String::from_utf8_lossy(&numi);
    let num_images: usize = numi_str
        .parse()
        .expect("Image count string cannot be coerced to a number");

    // for img in 0..num_images {
    //     let img

    println!("File Type: {}", file_type_str);
    println!("NITF Version: {}", version_str);
    println!("C Level: {}", c_level_str);
    println!("S Type: {}", s_type_str);
    println!("OSTAID: {}", ostaid_str);
    println!("FDT: {}", fdt_str);
    println!("FTitle: {}", ftitle_str);
    println!("FSCLAS: {}", fsclas_str);
    println!("FSCLSY: {}", fsclsy_str);
    println!("FSCode: {}", fscode_str);
    println!("FSCTLH: {}", fsctlh_str);
    println!("FSREL: {}", fsrel_str);
    println!("FSSCTP: {}", fsdctp_str);
    println!("FSSCDT: {}", fsdcdt_str);
    println!("FSDCXM: {}", fsdcxm_str);
    println!("FSDG: {}", fsdg_str);
    println!("FSDGDT: {}", fsdgdt_str);
    println!("FSCLTX: {}", fscltx_str);
    println!("FSCATP: {}", fscatp_str);
    println!("FSCAUT: {}", fscaut_str);
    println!("FSCRSN: {}", fscrsn_str);
    println!("FSSRDT: {}", fssrdt_str);
    println!("FSCTLN: {}", fsctln_str);
    println!("FSCOP: {}", fscop_str);
    println!("FSCPYS: {}", fscpys_str);
    println!("ENCRYP: {}", encryp_str);
    println!("FBKGC: {}", fbkgc_str);
    println!("ONAME: {}", oname_str);
    println!("OPHONE: {}", ophone_str);
    println!("FL: {}", fl_str);
    println!("HL: {}", hl_str);
    println!("NUMI: {}", numi_str);
    if !file_type_str.starts_with("NITF") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a valid NITF file",
        ));
    }

    println!("NUMI AS NUMBER: {}", num_images);

    //Read Images
    if num_images > 0 {
        let mut image_header_lens = std::vec::Vec::new(); 
        let mut image_data_lens = std::vec::Vec::new(); 
        for img in 0..num_images {
            let img_no=img+1;
            let mut lish = [0u8; LISHn];
            let mut lin = [0u8; LIn];
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
        let _ = file.seek(SeekFrom::Start(hl_str.parse().expect("Failed to coerce Header Length to u64")));
        for img in 0..num_images {
            let mut img_header = vec![0u8; image_header_lens[img]]; 
            let _ = file.read_exact(&mut img_header);
            let mut img_data = vec![0u8; image_data_lens[img]];
            let _ = file.read_exact(&mut img_data);
            let path = format!("img{}.jp2", img);
            let mut out_file = File::create(path)?;
            let _ = out_file.write_all(&img_data);
        };
    }
    Ok((0,0))
}
