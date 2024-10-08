use crate::modify::parser::file_ops::{read_int_from_file, read_int_from_bytes, read_string_from_file};
use std::fs::File;

const FHDR: usize = 4;
const FVER: usize = 5;

#[derive(Default, Debug)]
pub struct Nitf {
    pub fhdr: usize,
    pub fver: usize,
    pub clevel: usize,
    pub stype: usize,
    pub ostaid: usize,
    pub fdt: usize,
    pub ftitle: usize,
    pub fsclas: usize,
    pub fsclsy: usize,
    pub fscode: usize,
    pub fsctlh: usize,
    pub fsrel: usize,
    pub fsdctp: usize,
    pub fsdcdt: usize,
    pub fsdcxm: usize,
    pub fsdg: usize,
    pub fsdgdt: usize,
    pub fscltx: usize,
    pub fscatp: usize,
    pub fscaut: usize,
    pub fscrsn: usize,
    pub fssrdt: usize,
    pub fsctln: usize,
    pub fsdwng: usize,
    pub fsdevt: usize,
    pub fscop: usize,
    pub fscpys: usize,
    pub encryp: usize,
    pub fbkgc: usize,
    pub oname: usize,
    pub ophone: usize,
    pub fl: usize,
    pub hl: usize,
    pub numi: usize,
    pub lish_nnn: usize,
    pub li_nnn: usize,
    // non-header values
    pub file_profile_name: String,
    pub file_version: String,
    pub header_length: usize,
    pub num_images: usize,
    pub image_header_lengths: Vec<usize>,
    pub image_data_lengths: Vec<usize>,
}

impl Nitf {
    pub fn get_file_profile_name_and_version(self, file: &File) -> Nitf {
        let file_profile_name = read_string_from_file(file, 0, FHDR);
        let file_version = read_string_from_file(file, FHDR, FVER);
        Nitf {
            file_profile_name,
            file_version,
            ..self
        }
    }

    pub fn load_v02_10(self) -> Nitf {
        Nitf {
            fhdr: 4,
            fver: 5,
            clevel: 2,
            stype: 4,
            ostaid: 10,
            fdt: 14,
            ftitle: 80,
            fsclas: 1,
            fsclsy: 2,
            fscode: 11,
            fsctlh: 2,
            fsrel: 20,
            fsdctp: 2,
            fsdcdt: 8,
            fsdcxm: 4,
            fsdg: 1,
            fsdgdt: 8,
            fscltx: 43,
            fscatp: 1,
            fscaut: 40,
            fscrsn: 1,
            fssrdt: 8,
            fsctln: 15,
            fscop: 5,
            fscpys: 5,
            encryp: 1,
            fbkgc: 3,
            oname: 24,
            ophone: 18,
            fl: 12,
            hl: 6,
            numi: 3,
            lish_nnn: 6,
            li_nnn: 10,
            ..self
        }
    }

    pub fn load_v02_00(self) -> Nitf {
        Nitf {
            fhdr: 9,
            clevel: 2,
            stype: 4,
            ostaid: 10,
            fdt: 14,
            ftitle: 80,
            fsclas: 1,
            fscode: 40,
            fsctlh: 40,
            fsrel: 40,
            fscaut: 20,
            fsctln: 20,
            fsdwng: 6,
            fsdevt: 40,
            fscop: 5,
            fscpys: 5,
            encryp: 1,
            oname: 27,
            ophone: 18,
            fl: 12,
            hl: 6,
            numi: 3,
            lish_nnn: 6,
            li_nnn: 10,
            ..self
        }
    }
}

#[derive(Debug)]
pub enum NitfHeader21 {
    FHDR,
    FVER,
    CLEVEL,
    STYPE,
    OSTAID,
    FDT,
    FTITLE,
    FSCLAS,
    FSCLSY,
    FSCODE,
    FSCTLH,
    FSREL,
    FSDCTP,
    FSDCDT,
    FSDCXM,
    FSDG,
    FSDGDT,
    FSCLTX,
    FSCATP,
    FSCAUT,
    FSCRSN,
    FSSRDT,
    FSCTLN,
    FSCOP,
    FSCPYS,
    ENCRYP,
    FBKGC,
    ONAME,
    OPHONE,
    FL,
    HL,
    NUMI,
    LISHNNN,
    LINNN,
    NUMS,
    LSSHNNN,
    LSNNN,
    NUMX,
    NUMT,
    LTSHNNN,
    LTNNN,
    NUMDES,
    LDSHNNN,
    LDNNN,
    NUMRES,
    LRESHNNN,
    LRENNN,
    UDHDL,
    UDHOFL,
    UDHD,
    XHDL,
    XHDLOFL,
    XHD,
}

impl NitfHeader21 {
    pub fn values() -> &'static [usize] {
        &[
            4, 5, 2, 4, 10, 14, 80, 1, 2, 11, 2, 20, 2, 8, 4, 1, 8, 43, 1, 40, 1, 8, 15, 5, 5, 1,
            3, 24, 18, 12, 6, 3, 6, 10, 3, 4, 6, 3, 3, 4, 5, 3, 4, 9, 3, 4, 7, 5, 3, 0, 5, 3, 0
        ]
    }
    pub fn get_value(target: NitfHeader21) -> usize {
        let index = target as usize;
        NitfHeader21::values()[index]
    }
    pub fn get_offset(target: NitfHeader21, nitf_file: Option<&File>) -> usize {
        use NitfHeader21::{self as N, *};
        match target {
            //Graphic Segment offsets
            NUMS => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let base_offset = N::get_offset(NUMI, None);
                let num_images = read_int_from_file(nitf_file.expect("Dev Error, this should not be called with no file."), base_offset, N::get_value(NUMI));
                base_offset
                    + N::get_value(NUMI)
                    + (num_images * (N::get_value(LISHNNN) + N::get_value(LINNN)))
            }
            LSSHNNN  => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(NUMS, nitf_file) + N::get_value(NUMS)
            }
            LSNNN => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(LSSHNNN, nitf_file) + N::get_value(LSSHNNN)
            }
            //NUMX IS RESERVED FOR FUTURE USE
            NUMX => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let lsshnnn_offset = N::get_offset(LSSHNNN, nitf_file);
                let num_segments = read_int_from_file(nitf_file.expect("You must provide a file to get the offset of this header field."), N::get_offset(NUMS, nitf_file), N::get_value(NUMS));
                lsshnnn_offset + (num_segments * (N::get_value(LSSHNNN) + N::get_value(LSNNN)))
            }
            //Text Segment offsets
            NUMT => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let numx_offset = N::get_offset(NUMX, nitf_file);
                numx_offset + N::get_value(NUMX)
            }
            LTSHNNN => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(NUMT, nitf_file) + N::get_value(NUMT)
            }
            LTNNN => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(LTSHNNN, nitf_file) + N::get_value(LTSHNNN)
            }
            //Data Extension Segment offsets
            NUMDES => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let num_text = read_int_from_file(nitf_file.expect("You must provide a file to get the offset of this header field."), N::get_offset(NUMT, nitf_file), N::get_value(NUMT));
                N::get_offset(LTSHNNN, nitf_file) + (num_text * (N::get_value(LTSHNNN) + N::get_value(LTNNN)))
            }
            LDSHNNN => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(NUMDES, nitf_file) + N::get_value(NUMDES)
            }
            LDNNN => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(LDSHNNN, nitf_file) + N::get_value(LDSHNNN)
            }
            //Reserved Extension Segment offsets
            NUMRES => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let num_des = read_int_from_file(nitf_file.expect("You must provide a file to get the offset of this header field."), N::get_offset(NUMDES, nitf_file), N::get_value(NUMDES));
                N::get_offset(NUMDES, nitf_file) + (num_des * (N::get_value(LDSHNNN) + N::get_value(LDNNN)))
            }
            LRESHNNN => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(NUMRES, nitf_file) + N::get_value(NUMRES)
            }
            LRENNN => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(LRESHNNN, nitf_file) + N::get_value(LRESHNNN)
            }
            //User Defined Header offsets
            UDHDL => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let num_res = read_int_from_file(nitf_file.expect("You must provide a file to get the offset of this header field."), N::get_offset(NUMRES, nitf_file), N::get_value(NUMRES));
                N::get_offset(NUMRES, nitf_file) + (num_res * (N::get_value(LRESHNNN) + N::get_value(LRENNN)))
            }
            UDHOFL => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(UDHDL, nitf_file) + N::get_value(UDHDL)
            }
            UDHD => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(UDHOFL, nitf_file) + N::get_value(UDHOFL)
            }
            //Extended Header Data Length Segment offsets
            XHDL => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let udhd_len = read_int_from_file(nitf_file.expect("You must provide a file to get the offset of this header field."), N::get_offset(UDHDL, nitf_file), N::get_value(UDHDL));
                N::get_offset(UDHOFL, nitf_file) + udhd_len
            }
            XHDLOFL => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(XHDL, nitf_file) + N::get_value(XHDL)
            }
            XHD => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let xhd_len = read_int_from_file(nitf_file.expect("You must provide a file to get the offset of this header field."), N::get_offset(XHDL, nitf_file), N::get_value(XHDL));
                N::get_offset(XHDLOFL, nitf_file) + xhd_len
            }
            //Anything before NUMS
            _ => {
                let index = target as usize;
                N::values()[..index].iter().sum()
            }
        }
    }

    pub fn get_offset_bytes(target: NitfHeader21, bytes: Option<&Vec<u8>>) -> usize {
        use NitfHeader21::{self as N, *};
        match target {
            //Graphic Segment offsets
            NUMS => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let base_offset = N::get_offset_bytes(NUMI, None);
                let num_images = read_int_from_bytes(bytes.expect("Dev Error, this should not be called with no file."), base_offset, N::get_value(NUMI));
                base_offset
                    + N::get_value(NUMI)
                    + (num_images * (N::get_value(LISHNNN) + N::get_value(LINNN)))
            }
            LSSHNNN  => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(NUMS, bytes) + N::get_value(NUMS)
            }
            LSNNN => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(LSSHNNN, bytes) + N::get_value(LSSHNNN)
            }
            //NUMX IS RESERVED FOR FUTURE USE
            NUMX => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let lsshnnn_offset = N::get_offset_bytes(LSSHNNN, bytes);
                let num_segments = read_int_from_bytes(bytes.expect("You must provide a file to get the offset of this header field."), N::get_offset_bytes(NUMS, bytes), N::get_value(NUMS));
                lsshnnn_offset + (num_segments * (N::get_value(LSSHNNN) + N::get_value(LSNNN)))
            }
            //Text Segment offsets
            NUMT => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let numx_offset = N::get_offset_bytes(NUMX, bytes);
                numx_offset + N::get_value(NUMX)
            }
            LTSHNNN => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(NUMT, bytes) + N::get_value(NUMT)
            }
            LTNNN => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(LTSHNNN, bytes) + N::get_value(LTSHNNN)
            }
            //Data Extension Segment offsets
            NUMDES => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let num_text = read_int_from_bytes(bytes.expect("You must provide a file to get the offset of this header field."), N::get_offset_bytes(NUMT, bytes), N::get_value(NUMT));
                N::get_offset_bytes(LTSHNNN, bytes) + (num_text * (N::get_value(LTSHNNN) + N::get_value(LTNNN)))
            }
            LDSHNNN => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(NUMDES, bytes) + N::get_value(NUMDES)
            }
            LDNNN => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(LDSHNNN, bytes) + N::get_value(LDSHNNN)
            }
            //Reserved Extension Segment offsets
            NUMRES => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let num_des = read_int_from_bytes(bytes.expect("You must provide a file to get the offset of this header field."), N::get_offset_bytes(NUMDES, bytes), N::get_value(NUMDES));
                N::get_offset_bytes(NUMDES, bytes) + (num_des * (N::get_value(LDSHNNN) + N::get_value(LDNNN)))
            }
            LRESHNNN => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(NUMRES, bytes) + N::get_value(NUMRES)
            }
            LRENNN => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(LRESHNNN, bytes) + N::get_value(LRESHNNN)
            }
            //User Defined Header offsets
            UDHDL => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let num_res = read_int_from_bytes(bytes.expect("You must provide a file to get the offset of this header field."), N::get_offset_bytes(NUMRES, bytes), N::get_value(NUMRES));
                N::get_offset_bytes(NUMRES, bytes) + (num_res * (N::get_value(LRESHNNN) + N::get_value(LRENNN)))
            }
            UDHOFL => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(UDHDL, bytes) + N::get_value(UDHDL)
            }
            UDHD => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(UDHOFL, bytes) + N::get_value(UDHOFL)
            }
            //Extended Header Data Length Segment offsets
            XHDL => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let udhd_len = read_int_from_bytes(bytes.expect("You must provide a file to get the offset of this header field."), N::get_offset_bytes(UDHDL, bytes), N::get_value(UDHDL));
                N::get_offset_bytes(UDHOFL, bytes) + udhd_len
            }
            XHDLOFL => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset_bytes(XHDL, bytes) + N::get_value(XHDL)
            }
            XHD => {
                if bytes.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let xhd_len = read_int_from_bytes(bytes.expect("You must provide a file to get the offset of this header field."), N::get_offset_bytes(XHDL, bytes), N::get_value(XHDL));
                N::get_offset_bytes(XHDLOFL, bytes) + xhd_len
            }
            //Anything before NUMS
            _ => {
                let index = target as usize;
                let val = N::values()[..index].iter().sum();
                val
            }
        }
    }

    pub fn get_image_header_field_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let lish_nnn_offset = N::get_offset(LISHNNN, nitf_file);
        lish_nnn_offset + (image_num as usize * (N::get_value(LISHNNN) + N::get_value(LINNN)))
    }

    pub fn get_image_header_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_bytes(bytes, N::get_offset_bytes(NUMI, nitf_bytes), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let lish_nnn_offset = N::get_offset_bytes(LISHNNN, nitf_bytes);
        lish_nnn_offset + (image_num as usize * (N::get_value(LISHNNN) + N::get_value(LINNN)))
    }

    pub fn get_image_data_field_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let lish_nnn_offset = N::get_image_header_field_offset(nitf_file, image_num);
        lish_nnn_offset + N::get_value(LISHNNN)
    }

    pub fn get_image_data_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_bytes(bytes, N::get_offset_bytes(NUMI, nitf_bytes), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let lish_nnn_offset = N::get_image_header_field_offset_bytes(nitf_bytes, image_num);
        lish_nnn_offset + N::get_value(LISHNNN)
    }

    pub fn get_graphic_header_field_offset(nitf_file: Option<&File>, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_graphics = read_int_from_file(file, N::get_offset(NUMS, nitf_file), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lssh_nnn_offset = N::get_offset(LSSHNNN, nitf_file);
        lssh_nnn_offset + (graphic_num as usize * (N::get_value(LSSHNNN) + N::get_value(LSNNN)))
    }
    
    pub fn get_graphic_header_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_graphics = read_int_from_bytes(bytes, N::get_offset_bytes(NUMS, nitf_bytes), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 {panic!("Graphic number is greater than the number of graphics in the bytes, they are 0 indexed, are you off by 1?");}
        let lssh_nnn_offset = N::get_offset_bytes(LSSHNNN, nitf_bytes);
        lssh_nnn_offset + (graphic_num as usize * (N::get_value(LSSHNNN) + N::get_value(LSNNN)))
    }

    pub fn get_graphic_data_field_offset(nitf_file: Option<&File>, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_graphics = read_int_from_file(file, N::get_offset(NUMS, nitf_file), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lssh_nnn_offset = N::get_graphic_header_field_offset(nitf_file, graphic_num);
        lssh_nnn_offset + N::get_value(LSSHNNN) 
    }

    pub fn get_graphic_data_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_graphics = read_int_from_bytes(bytes, N::get_offset_bytes(NUMS, nitf_bytes), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lssh_nnn_offset = N::get_graphic_header_field_offset_bytes(nitf_bytes, graphic_num);
        lssh_nnn_offset + N::get_value(LSSHNNN) 
    }

    pub fn get_text_header_field_offset(nitf_file: Option<&File>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_file(file, N::get_offset(NUMT, nitf_file), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ltsh_nnn_offset = N::get_offset(LTSHNNN, nitf_file);
        ltsh_nnn_offset + (text_num as usize * (N::get_value(LTSHNNN) + N::get_value(LTNNN)))
    }

    pub fn get_text_header_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_bytes(bytes, N::get_offset_bytes(NUMT, nitf_bytes), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ltsh_nnn_offset = N::get_offset_bytes(LTSHNNN, nitf_bytes);
        ltsh_nnn_offset + (text_num as usize * (N::get_value(LTSHNNN) + N::get_value(LTNNN)))
    }

    pub fn get_text_data_field_offset(nitf_file: Option<&File>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_file(file, N::get_offset(NUMT, nitf_file), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ltsh_nnn_offset = N::get_text_header_field_offset(nitf_file, text_num);
        ltsh_nnn_offset + N::get_value(LTSHNNN) 
    }

    pub fn get_text_data_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_bytes(bytes, N::get_offset_bytes(NUMT, nitf_bytes), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ltsh_nnn_offset = N::get_text_header_field_offset_bytes(nitf_bytes, text_num);
        ltsh_nnn_offset + N::get_value(LTSHNNN) 
    }
    
    /// Get the offset of the Data Extension Segment header length header field
    pub fn get_des_header_field_offset(nitf_file: Option<&File>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_des = read_int_from_file(file, N::get_offset(NUMDES, nitf_file), N::get_value(NUMDES));
        if des_num + 1 > num_des as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ldsh_nnn_offset = N::get_offset(LDSHNNN, nitf_file);
        ldsh_nnn_offset + (des_num as usize * (N::get_value(LDSHNNN) + N::get_value(LDNNN)))
    }
    
    pub fn get_des_header_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_des = read_int_from_bytes(bytes, N::get_offset_bytes(NUMDES, nitf_bytes), N::get_value(NUMDES));
        if des_num + 1 > num_des as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ldsh_nnn_offset = N::get_offset_bytes(LDSHNNN, nitf_bytes);
        ldsh_nnn_offset + (des_num as usize * (N::get_value(LDSHNNN) + N::get_value(LDNNN)))
    }
    
    /// Get the offset of the Data Extension Segment data length header field
    pub fn get_des_data_field_offset(nitf_file: Option<&File>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_des = read_int_from_file(file, N::get_offset(NUMDES, nitf_file), N::get_value(NUMDES));
        if des_num + 1 > num_des as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ldsh_nnn_offset = N::get_des_header_field_offset(nitf_file, des_num);
        ldsh_nnn_offset + N::get_value(LDSHNNN) 
    }

    pub fn get_des_data_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_des = read_int_from_bytes(bytes, N::get_offset_bytes(NUMDES, nitf_bytes), N::get_value(NUMDES));
        if des_num + 1 > num_des as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ldsh_nnn_offset = N::get_des_header_field_offset_bytes(nitf_bytes, des_num);
        ldsh_nnn_offset + N::get_value(LDSHNNN) 
    }

    pub fn get_reserved_header_field_offset(nitf_file: Option<&File>, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_res = read_int_from_file(file, N::get_offset(NUMRES, nitf_file), N::get_value(NUMRES));
        if res_num + 1 > num_res as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lresh_nnn_offset = N::get_offset(LRESHNNN, nitf_file);
        lresh_nnn_offset + (res_num as usize * (N::get_value(LRESHNNN) + N::get_value(LRENNN)))
    }

    pub fn get_reserved_header_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_res = read_int_from_bytes(bytes, N::get_offset_bytes(NUMRES, nitf_bytes), N::get_value(NUMRES));
        if res_num + 1 > num_res as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lresh_nnn_offset = N::get_offset_bytes(LRESHNNN, nitf_bytes);
        lresh_nnn_offset + (res_num as usize * (N::get_value(LRESHNNN) + N::get_value(LRENNN)))
    }

    pub fn get_reserved_data_field_offset(nitf_file: Option<&File>, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_res = read_int_from_file(file, N::get_offset(NUMRES, nitf_file), N::get_value(NUMRES));
        if res_num + 1 > num_res as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lresh_nnn_offset = N::get_reserved_header_field_offset(nitf_file, res_num);
        lresh_nnn_offset + N::get_value(LRESHNNN) 
    }

    pub fn get_reserved_data_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_res = read_int_from_bytes(bytes, N::get_offset_bytes(NUMRES, nitf_bytes), N::get_value(NUMRES));
        if res_num + 1 > num_res as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lresh_nnn_offset = N::get_reserved_header_field_offset_bytes(nitf_bytes, res_num);
        lresh_nnn_offset + N::get_value(LRESHNNN) 
    }

    pub fn get_image_subheader_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let header_length = read_int_from_file(file, N::get_offset(HL, nitf_file), N::get_value(HL));
        if image_num == 0 { return header_length }
        let mut offset = header_length;
        for i in 0..image_num {
            offset += read_int_from_file(file, N::get_image_header_field_offset(nitf_file, i), N::get_value(LISHNNN))
                + read_int_from_file(file, N::get_image_data_field_offset(nitf_file, i), N::get_value(LINNN));
        }
        offset
    }

    pub fn get_image_subheader_offset_bytes(nitf_bytes: Option<&Vec<u8>>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_bytes(bytes, N::get_offset_bytes(NUMI, nitf_bytes), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let header_length = read_int_from_bytes(bytes, N::get_offset_bytes(HL, nitf_bytes), N::get_value(HL));
        if image_num == 0 { return header_length }
        let mut offset = header_length;
        for i in 0..image_num {
            offset += read_int_from_bytes(bytes, N::get_image_header_field_offset_bytes(nitf_bytes, i), N::get_value(LISHNNN))
                + read_int_from_bytes(bytes, N::get_image_data_field_offset_bytes(nitf_bytes, i), N::get_value(LINNN));
        }
        offset
    }

    pub fn get_image_data_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let image_subheader_length = read_int_from_file(file, N::get_image_subheader_offset(nitf_file, image_num), N::get_value(LISHNNN));
        N::get_image_subheader_offset(nitf_file, image_num) + image_subheader_length
    }

    pub fn get_image_data_offset_bytes(nitf_bytes: Option<&Vec<u8>>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes = nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_bytes(bytes, N::get_offset_bytes(NUMI, nitf_bytes), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let image_subheader_length = read_int_from_bytes(bytes, N::get_image_subheader_offset_bytes(nitf_bytes, image_num), N::get_value(LISHNNN));
        N::get_image_subheader_offset_bytes(nitf_bytes, image_num) + image_subheader_length
    }

    pub fn get_images_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        read_int_from_file(nitf_file, N::get_offset(HL, Some(nitf_file)), N::get_value(HL))
    }

    pub fn get_images_segment_start_bytes(bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        read_int_from_bytes(bytes, N::get_offset_bytes(HL, Some(bytes)), N::get_value(HL))
    }
    
    pub fn get_graphic_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let num_images = read_int_from_file(nitf_file, N::get_offset(NUMI, Some(nitf_file)), N::get_value(NUMI));
        let mut offset = N::get_images_segment_start(nitf_file);
        for i in 0..num_images {
            offset += read_int_from_file(nitf_file, N::get_image_header_field_offset(Some(nitf_file), i as u64), N::get_value(LISHNNN))
                + read_int_from_file(nitf_file, N::get_image_data_field_offset(Some(nitf_file), i as u64), N::get_value(LINNN));
        }
        offset
    }

    pub fn get_graphic_segment_start_bytes(bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        let num_images = read_int_from_bytes(bytes, N::get_offset_bytes(NUMI, Some(bytes)), N::get_value(NUMI));
        let mut offset = N::get_images_segment_start_bytes(bytes);
        for i in 0..num_images {
            offset += read_int_from_bytes(bytes, N::get_image_header_field_offset_bytes(Some(bytes), i as u64), N::get_value(LISHNNN))
                + read_int_from_bytes(bytes, N::get_image_data_field_offset_bytes(Some(bytes), i as u64), N::get_value(LINNN));
        }
        offset
    }

    pub fn get_reserved_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21 as N;
        println!("This segment is reserved for a future standard. This method currently returns the same value as get_text_segment_start.");
        N::get_text_segment_start(nitf_file)
    }

    pub fn get_reserved_segment_start_bytes(nitf_bytes: &Vec<u8>) -> usize {
        use NitfHeader21 as N;
        println!("This segment is reserved for a future standard. This method currently returns the same value as get_text_segment_start.");
        N::get_text_segment_start_bytes(nitf_bytes)
    }

    pub fn get_text_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let num_graphics = read_int_from_file(nitf_file, N::get_offset(NUMS, Some(nitf_file)), N::get_value(NUMS));
        let mut offset = N::get_graphic_segment_start(nitf_file);
        for i in 0..num_graphics {
            offset += read_int_from_file(nitf_file, N::get_image_header_field_offset(Some(nitf_file), i as u64), N::get_value(LISHNNN))
                + read_int_from_file(nitf_file, N::get_image_data_field_offset(Some(nitf_file), i as u64), N::get_value(LINNN));
        }
        offset
    }

    pub fn get_text_segment_start_bytes(nitf_bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        let num_graphics = read_int_from_bytes(nitf_bytes, N::get_offset_bytes(NUMS, Some(nitf_bytes)), N::get_value(NUMS));
        let mut offset = N::get_graphic_segment_start_bytes(nitf_bytes);
        for i in 0..num_graphics {
            offset += read_int_from_bytes(nitf_bytes, N::get_image_header_field_offset_bytes(Some(nitf_bytes), i as u64), N::get_value(LISHNNN))
                + read_int_from_bytes(nitf_bytes, N::get_image_data_field_offset_bytes(Some(nitf_bytes), i as u64), N::get_value(LINNN));
        }
        offset
    }
    
    pub fn get_des_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let num_text = read_int_from_file(nitf_file, N::get_offset(NUMT, Some(nitf_file)), N::get_value(NUMT));
        let mut offset = N::get_text_segment_start(nitf_file);
        for i in 0..num_text {
            offset += read_int_from_file(nitf_file, N::get_text_header_field_offset(Some(nitf_file), i as u64), N::get_value(LTSHNNN))
                + read_int_from_file(nitf_file, N::get_text_data_field_offset(Some(nitf_file), i as u64), N::get_value(LTNNN));
        }
        offset
    }

    pub fn get_des_segment_start_bytes(nitf_bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        let num_text = read_int_from_bytes(nitf_bytes, N::get_offset_bytes(NUMT, Some(nitf_bytes)), N::get_value(NUMT));
        let mut offset = N::get_text_segment_start_bytes(nitf_bytes);
        for i in 0..num_text {
            offset += read_int_from_bytes(nitf_bytes, N::get_text_header_field_offset_bytes(Some(nitf_bytes), i as u64), N::get_value(LTSHNNN))
                + read_int_from_bytes(nitf_bytes, N::get_text_data_field_offset_bytes(Some(nitf_bytes), i as u64), N::get_value(LTNNN));
        }
        offset
    }

    pub fn get_reserved_extension_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let num_des = read_int_from_file(nitf_file, N::get_offset(NUMDES, Some(nitf_file)), N::get_value(NUMDES));
        let mut offset = N::get_des_segment_start(nitf_file);
        for i in 0..num_des {
            offset += read_int_from_file(nitf_file, N::get_des_header_field_offset(Some(nitf_file), i as u64), N::get_value(LDSHNNN))
                + read_int_from_file(nitf_file, N::get_des_data_field_offset(Some(nitf_file), i as u64), N::get_value(LDNNN));
        }
        offset
    }

    pub fn get_reserved_extension_segment_start_bytes(nitf_bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        let num_des = read_int_from_bytes(nitf_bytes, N::get_offset_bytes(NUMDES, Some(nitf_bytes)), N::get_value(NUMDES));
        let mut offset = N::get_des_segment_start_bytes(nitf_bytes);
        for i in 0..num_des {
            offset += read_int_from_bytes(nitf_bytes, N::get_des_header_field_offset_bytes(Some(nitf_bytes), i as u64), N::get_value(LDSHNNN))
                + read_int_from_bytes(nitf_bytes, N::get_des_data_field_offset_bytes(Some(nitf_bytes), i as u64), N::get_value(LDNNN));
        }
        offset
    }

    pub fn get_image_segments_offset(nitf_file: &File, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_images = read_int_from_file(nitf_file, N::get_offset(NUMI, Some(nitf_file)), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 && image_num != 0 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_images_segment_start(nitf_file);
        for i in 0..image_num {
            offset += read_int_from_file(nitf_file, N::get_image_header_field_offset(Some(nitf_file), i), N::get_value(LISHNNN))
                + read_int_from_file(nitf_file, N::get_image_data_field_offset(Some(nitf_file), i), N::get_value(LINNN));
        }
        offset
    }

    pub fn get_image_segments_offset_bytes(nitf_bytes: &Vec<u8>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_images = read_int_from_bytes(nitf_bytes, N::get_offset_bytes(NUMI, Some(nitf_bytes)), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 && image_num != 0 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_images_segment_start_bytes(nitf_bytes);
        for i in 0..image_num {
            offset += read_int_from_bytes(nitf_bytes, N::get_image_header_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LISHNNN))
                + read_int_from_bytes(nitf_bytes, N::get_image_data_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LINNN));
        }
        offset
    }

    pub fn get_graphic_segments_offset(nitf_file: &File, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_graphics = read_int_from_file(nitf_file, N::get_offset(NUMS, Some(nitf_file)), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 && graphic_num != 0 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_graphic_segment_start(nitf_file);
        for i in 0..graphic_num {
            offset += read_int_from_file(nitf_file, N::get_graphic_header_field_offset(Some(nitf_file), i), N::get_value(LSSHNNN))
                + read_int_from_file(nitf_file, N::get_graphic_data_field_offset(Some(nitf_file), i), N::get_value(LSNNN));
        }
        offset
    }

    pub fn get_graphic_segments_offset_bytes(nitf_bytes: &Vec<u8>, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_graphics = read_int_from_bytes(nitf_bytes, N::get_offset_bytes(NUMS, Some(nitf_bytes)), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 && graphic_num != 0 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_graphic_segment_start_bytes(nitf_bytes);
        for i in 0..graphic_num {
            offset += read_int_from_bytes(nitf_bytes, N::get_graphic_header_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LSSHNNN))
                + read_int_from_bytes(nitf_bytes, N::get_graphic_data_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LSNNN));
        }
        offset
    }

    pub fn get_text_segments_offset(nitf_file: &File, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_text = read_int_from_file(nitf_file, N::get_offset(NUMT, Some(nitf_file)), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 && text_num != 0 {panic!("Text number is greater than the number of text segments in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_offset(NUMT, Some(nitf_file)) + N::get_value(NUMT);
        for i in 0..text_num {
            offset += read_int_from_file(nitf_file, N::get_text_header_field_offset(Some(nitf_file), i), N::get_value(LTSHNNN))
                + read_int_from_file(nitf_file, N::get_text_data_field_offset(Some(nitf_file), i), N::get_value(LTNNN));
        }
        offset
    }

    pub fn get_text_segments_offset_bytes(nitf_bytes: &Vec<u8>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_text = read_int_from_bytes(nitf_bytes, N::get_offset_bytes(NUMT, Some(nitf_bytes)), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 && text_num != 0 {panic!("Text number is greater than the number of text segments in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_offset_bytes(NUMT, Some(nitf_bytes)) + N::get_value(NUMT);
        for i in 0..text_num {
            offset += read_int_from_bytes(nitf_bytes, N::get_text_header_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LTSHNNN))
                + read_int_from_bytes(nitf_bytes, N::get_text_data_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LTNNN));
        }
        offset
    }

    pub fn get_des_segments_offset(nitf_file: &File, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_des = read_int_from_file(nitf_file, N::get_offset(NUMDES, Some(nitf_file)), N::get_value(NUMDES));
        if des_num + 1 > num_des as u64 && des_num != 0 {panic!("Data Extension number is greater than the number of data extension segments in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_des_segment_start(nitf_file);
        for i in 0..des_num {
            offset += read_int_from_file(nitf_file, N::get_des_header_field_offset(Some(nitf_file), i), N::get_value(LDSHNNN))
                + read_int_from_file(nitf_file, N::get_des_data_field_offset(Some(nitf_file), i), N::get_value(LDNNN));
        }
        offset
    }

    pub fn get_des_segments_offset_bytes(nitf_bytes: &Vec<u8>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_des = read_int_from_bytes(nitf_bytes, N::get_offset_bytes(NUMDES, Some(nitf_bytes)), N::get_value(NUMDES));
        if des_num + 1 > num_des as u64 && des_num != 0 {panic!("Data Extension number is greater than the number of data extension segments in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_des_segment_start_bytes(nitf_bytes);
        for i in 0..des_num {
            offset += read_int_from_bytes(nitf_bytes, N::get_des_header_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LDSHNNN))
                + read_int_from_bytes(nitf_bytes, N::get_des_data_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LDNNN));
        }
        offset
    }

    pub fn get_reserved_segments_offset(nitf_file: &File, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_res = read_int_from_file(nitf_file, N::get_offset(NUMRES, Some(nitf_file)), N::get_value(NUMRES));
        if res_num + 1 > num_res as u64 && res_num != 0 {panic!("Reserved Extension number is greater than the number of reserved extension segments in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_reserved_extension_segment_start(nitf_file);
        for i in 0..res_num {
            offset += read_int_from_file(nitf_file, N::get_reserved_header_field_offset(Some(nitf_file), i), N::get_value(LRESHNNN))
                + read_int_from_file(nitf_file, N::get_reserved_data_field_offset(Some(nitf_file), i), N::get_value(LRENNN));
        }
        offset
    }

    pub fn get_reserved_segments_offset_bytes(nitf_bytes: &Vec<u8>, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_res = read_int_from_bytes(nitf_bytes, N::get_offset_bytes(NUMRES, Some(nitf_bytes)), N::get_value(NUMRES));
        if res_num + 1 > num_res as u64 && res_num != 0 {panic!("Reserved Extension number is greater than the number of reserved extension segments in the file, they are 0 indexed, are you off by 1?");}
        let mut offset = N::get_reserved_extension_segment_start_bytes(nitf_bytes);
        for i in 0..res_num {
            offset += read_int_from_bytes(nitf_bytes, N::get_reserved_header_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LRESHNNN))
                + read_int_from_bytes(nitf_bytes, N::get_reserved_data_field_offset_bytes(Some(nitf_bytes), i), N::get_value(LRENNN));
        }
        offset
    }
}

pub enum NitfImageSubheader21 {
    IM,
    IID1,
    IIDATIM,
    TGTID,
    IID2,
    ISCLAS,
    ISCLSY,
    ISCODE,
    ISCTLH,
    ISREL,
    ISDCTP,
    ISDCDT,
    ISDCXM,
    ISDG,
    ISDGDT,
    ISCLTX,
    ISCATP,
    ISCAUT,
    ISCRSN,
    ISSRDT,
    ISCTLN,
    ENCRYP,
    ISORCE,
    NROWS,
    NCOLS,
    PVTYPE,
    IREP,
    ICAT,
    ABPP,
    PJUST,
    ICORDS,
    NICOM,
    ICOMNNN,
    IC,
    NBANDS,
    IREPBANDNNN,
    ISUBCATNNN,
    IFCNNN,
    IMFLTNNN,
    NLUTSNNN,
    ISYNC,
    IMODE,
    NBPR,
    NBPC,
    NPPBH,
    NPPBV,
    NBPP,
    IDLVL,
    IALVL,
    ILOC,
    IMAG,
    UDIDL,
    IXSHDL,
}

// impl NitfImageSubheader21 {
//     pub fn values() -> &'static [usize] {
//         &[2, 10, 14, 17, 80, 1, 2, 11, 2, 20, 2, 8, 4, 1, 8, 43, 1, 40, 1, 8, 15, 1, 42, 8, 8, 3, 4, 8, 2, 1, 1, 1, 80, 2, 1, 2, 6, 1, 3, 1, 1, 1, 4, 4, 4, 4, 2, 3, 3, 10, 4, 5, 5
//         ]
//     }
//     pub fn get_value(target: NitfImageSubheader21) -> usize {
//         let index = target as usize;
//         NitfImageSubheader21::values()[index]
//     }
// pub fn get_offset(target: NitfImageSubheader21, nitf_file: Option<&File>) -> usize {
//     match target {
//         NitfImageSubheader21::IC => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of X segments."
//                 ),
//             }
//             let base_offset = NitfImageSubheader21::get_offset(NitfImageSubheader21::NICOM, None);
//             let num_comments = read_int_from_file(
//                 &file,
//                 base_offset,
//                 NitfImageSubheader21::get_value(NitfImageSubheader21::NICOM),
//             );
//             base_offset
//                 + NitfImageSubheader21::get_value(NitfImageSubheader21::NICOM)
//                 + (num_comments
//                     * (NitfImageSubheader21::get_value(NitfImageSubheader21::ICOMNNN)))
//         }
//         NitfImageSubheader21::NUMX => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of Graphic segments."
//                 ),
//             }
//             let base_offset = NitfImageSubheader21::get_offset(NitfImageSubheader21::NUMS, Some(file));
//             let num_segments = read_int_from_file(
//                 &file,
//                 base_offset,
//                 NitfImageSubheader21::get_value(NitfImageSubheader21::NUMS),
//             );
//             base_offset
//                 + NitfImageSubheader21::get_value(NitfImageSubheader21::NUMS)
//                 + (num_segments
//                     * (NitfImageSubheader21::get_value(NitfImageSubheader21::LSSHNNN)
//                         + NitfImageSubheader21::get_value(NitfImageSubheader21::LSNNN)))
//         }
//         NitfImageSubheader21::NUMT => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of Text segments."
//                 ),
//             }
//             let base_offset = NitfImageSubheader21::get_offset(NitfImageSubheader21::NUMX, Some(file));
//             let num_text_segments = read_int_from_file(
//                 &file,
//                 base_offset,
//                 NitfImageSubheader21::get_value(NitfImageSubheader21::NUMX),
//             );
//             base_offset
//                 + NitfImageSubheader21::get_value(NitfImageSubheader21::NUMX)
//                 + (num_text_segments
//                     * (NitfImageSubheader21::get_value(NitfImageSubheader21::LTSHNNN)
//                         + NitfImageSubheader21::get_value(NitfImageSubheader21::LTNNN)))
//         }
//         NitfImageSubheader21::NUMDES => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of DESegments."
//                 ),
//             }
//             let header_length = read_int_from_file(
//                 &file,
//                 NitfImageSubheader21::get_offset(NitfImageSubheader21::HL, None),
//                 NitfImageSubheader21::get_value(NitfImageSubheader21::HL),
//             );
//             header_length
//                 - (NitfImageSubheader21::get_value(NitfImageSubheader21::XHDL)
//                     + NitfImageSubheader21::get_value(NitfImageSubheader21::UDHDL)
//                     + NitfImageSubheader21::get_value(NitfImageSubheader21::NUMRES)
//                     + NitfImageSubheader21::get_value(NitfImageSubheader21::NUMDES))
//         }
//         NitfImageSubheader21::NUMRES => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of segments."
//                 ),
//             }
//             let header_length = read_int_from_file(
//                 &file,
//                 NitfImageSubheader21::get_offset(NitfImageSubheader21::HL, None),
//                 NitfImageSubheader21::get_value(NitfImageSubheader21::HL),
//             );
//             header_length
//                 - (NitfImageSubheader21::get_value(NitfImageSubheader21::XHDL)
//                     + NitfImageSubheader21::get_value(NitfImageSubheader21::UDHDL)
//                     + NitfImageSubheader21::get_value(NitfImageSubheader21::NUMRES))
//         }
//         NitfImageSubheader21::UDHDL => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of segments."
//                 ),
//             }
//             let header_length = read_int_from_file(
//                 &file,
//                 NitfImageSubheader21::get_offset(NitfImageSubheader21::HL, None),
//                 NitfImageSubheader21::get_value(NitfImageSubheader21::HL),
//             );
//             header_length
//                 - (NitfImageSubheader21::get_value(NitfImageSubheader21::XHDL)
//                     + NitfImageSubheader21::get_value(NitfImageSubheader21::UDHDL))
//         }
//         NitfImageSubheader21::XHDL => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of segments."
//                 ),
//             }
//             let header_length = read_int_from_file(
//                 &file,
//                 NitfImageSubheader21::get_offset(NitfImageSubheader21::HL, None),
//                 NitfImageSubheader21::get_value(NitfImageSubheader21::HL),
//             );
//             header_length - NitfImageSubheader21::get_value(NitfImageSubheader21::XHDL)
//         }
//         NitfImageSubheader21::LISHNNN
//         | NitfImageSubheader21::LINNN
//         | NitfImageSubheader21::LSSHNNN
//         | NitfImageSubheader21::LSNNN
//         | NitfImageSubheader21::LTSHNNN
//         | NitfImageSubheader21::LTNNN => {
//             eprint!("You cannot get the single offset of N number of segments. Consider finding the offset of the number of segments and adding the size of the Subheader and Data length fields you are looking as many times as you need to get to the header offset you are looking for.");
//             0
//         }
//         _ => {
//             let index = target as usize;
//             NitfImageSubheader21::values()[..index].iter().sum()
//         }
//     }
//}
//}
