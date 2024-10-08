use crate::modify::parser::file_ops::read_string_from_file;
use crate::modify::parser::file_ops::read_int_from_file;
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
        println!("Reading file profile name from offset: 0");
        let file_profile_name = read_string_from_file(file, 0, FHDR);
        println!("Reading file version from offset: 4");
        let file_version = read_string_from_file(file, FHDR, FVER);

        Nitf {
            file_profile_name,
            file_version,
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
pub enum NitfHeader20 {
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
    NUMRES,
    UDHDL,
    XHDL,
}

impl NitfHeader20 {
    pub fn values() -> &'static [usize] {
        &[
            4, 5, 2, 4, 10, 14, 80, 1, 2, 11, 2, 20, 2, 8, 4, 1, 8, 43, 1, 40, 1, 8, 15, 5, 5, 1,
            3, 24, 18, 12, 6, 3, 6, 10, 3, 4, 6, 3, 3, 4, 5, 3, 3, 5, 5,
        ]
    }
    pub fn get_value(target: NitfHeader20) -> usize {
        let index = target as usize;
        NitfHeader20::values()[index]
    }
    pub fn get_offset(target: NitfHeader20, nitf_file: Option<&File>) -> usize {
        use NitfHeader20::{self as N, *};
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
                println!("Warning: This will only return the offset of the FIRST segment. Use get_<segment>_offset() to get the offset of a specific segment.");
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(NUMS, nitf_file) + N::get_value(NUMS)
            }
            LSNNN => {
                println!("Warning: This will only return the offset of the FIRST segment. Use get_<segment>_offset() to get the offset of a specific segment.");
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
                println!("Warning: This will only return the offset of the FIRST segment. Use get_<segment>_offset() to get the offset of a specific segment.");
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(NUMT, nitf_file) + N::get_value(NUMT)
            }
            LTNNN => {
                println!("Warning: This will only return the offset of the FIRST segment. Use get_<segment>_offset() to get the offset of a specific segment.");
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                N::get_offset(LTSHNNN, nitf_file) + N::get_value(LTSHNNN)
            }
            //Anything after the dynamic segmants
            XHDL | UDHDL | NUMRES | NUMDES => {
                if nitf_file.is_none() {panic!("You must provide a file to get the offset of this header field.");}
                let hl_offset = N::get_offset(HL, None);
                let header_length = read_int_from_file(
                    nitf_file.expect("You must provide a file to get the offset of this header field."),
                    hl_offset,
                    N::get_value(HL),
                );
                let index = target as usize;
                header_length - N::values()[index..].iter().sum::<usize>()
            }
            //Anything before NUMS
            _ => {
                let index = target as usize;
                N::values()[..index].iter().sum()
            }
        }
    }

    pub fn get_image_header_field_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader20::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let lish_nnn_offset = N::get_offset(LISHNNN, nitf_file);
        lish_nnn_offset + (image_num as usize * (N::get_value(LISHNNN) + N::get_value(LINNN)))
    }

    pub fn get_image_data_field_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader20::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let lish_nnn_offset = N::get_image_header_field_offset(nitf_file, image_num);
        lish_nnn_offset + N::get_value(LISHNNN)
    }

    pub fn get_graphic_header_field_offset(nitf_file: Option<&File>, graphic_num: u64) -> usize {
        use NitfHeader20::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_graphics = read_int_from_file(file, N::get_offset(NUMS, nitf_file), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lssh_nnn_offset = N::get_offset(LSSHNNN, nitf_file);
        lssh_nnn_offset + (graphic_num as usize * (N::get_value(LSSHNNN) + N::get_value(LSNNN)))
    }

    pub fn get_graphic_data_field_offset(nitf_file: Option<&File>, graphic_num: u64) -> usize {
        use NitfHeader20::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_graphics = read_int_from_file(file, N::get_offset(NUMS, nitf_file), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let lssh_nnn_offset = N::get_graphic_header_field_offset(nitf_file, graphic_num);
        lssh_nnn_offset + N::get_value(LSSHNNN) 
    }

    pub fn get_text_header_field_offset(nitf_file: Option<&File>, text_num: u64) -> usize {
        use NitfHeader20::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_file(file, N::get_offset(NUMT, nitf_file), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ltsh_nnn_offset = N::get_offset(LTSHNNN, nitf_file);
        ltsh_nnn_offset + (text_num as usize * (N::get_value(LTSHNNN) + N::get_value(LTNNN)))
    }

    pub fn get_text_data_field_offset(nitf_file: Option<&File>, text_num: u64) -> usize {
        use NitfHeader20::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_file(file, N::get_offset(NUMT, nitf_file), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 {panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");}
        let ltsh_nnn_offset = N::get_text_header_field_offset(nitf_file, text_num);
        ltsh_nnn_offset + N::get_value(LTSHNNN) 
    }

    pub fn get_image_subheader_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader20::{self as N, *};
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

    pub fn get_image_data_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader20::{self as N, *};
        let file = nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");}
        let image_subheader_length = read_int_from_file(file, N::get_image_subheader_offset(nitf_file, image_num), N::get_value(LISHNNN));
        N::get_image_subheader_offset(nitf_file, image_num) + image_subheader_length
    }
}

pub enum NitfImageSubheader20 {
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

// impl NitfImageSubheader20 {
//     pub fn values() -> &'static [usize] {
//         &[2, 10, 14, 17, 80, 1, 2, 11, 2, 20, 2, 8, 4, 1, 8, 43, 1, 40, 1, 8, 15, 1, 42, 8, 8, 3, 4, 8, 2, 1, 1, 1, 80, 2, 1, 2, 6, 1, 3, 1, 1, 1, 4, 4, 4, 4, 2, 3, 3, 10, 4, 5, 5
//         ]
//     }
//     pub fn get_value(target: NitfImageSubheader20) -> usize {
//         let index = target as usize;
//         NitfImageSubheader20::values()[index]
//     }
// pub fn get_offset(target: NitfImageSubheader20, nitf_file: Option<&File>) -> usize {
//     match target {
//         NitfImageSubheader20::IC => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of X segments."
//                 ),
//             }
//             let base_offset = NitfImageSubheader20::get_offset(NitfImageSubheader20::NICOM, None);
//             let num_comments = read_int_from_file(
//                 &file,
//                 base_offset,
//                 NitfImageSubheader20::get_value(NitfImageSubheader20::NICOM),
//             );
//             base_offset
//                 + NitfImageSubheader20::get_value(NitfImageSubheader20::NICOM)
//                 + (num_comments
//                     * (NitfImageSubheader20::get_value(NitfImageSubheader20::ICOMNNN)))
//         }
//         NitfImageSubheader20::NUMX => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of Graphic segments."
//                 ),
//             }
//             let base_offset = NitfImageSubheader20::get_offset(NitfImageSubheader20::NUMS, Some(file));
//             let num_segments = read_int_from_file(
//                 &file,
//                 base_offset,
//                 NitfImageSubheader20::get_value(NitfImageSubheader20::NUMS),
//             );
//             base_offset
//                 + NitfImageSubheader20::get_value(NitfImageSubheader20::NUMS)
//                 + (num_segments
//                     * (NitfImageSubheader20::get_value(NitfImageSubheader20::LSSHNNN)
//                         + NitfImageSubheader20::get_value(NitfImageSubheader20::LSNNN)))
//         }
//         NitfImageSubheader20::NUMT => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of Text segments."
//                 ),
//             }
//             let base_offset = NitfImageSubheader20::get_offset(NitfImageSubheader20::NUMX, Some(file));
//             let num_text_segments = read_int_from_file(
//                 &file,
//                 base_offset,
//                 NitfImageSubheader20::get_value(NitfImageSubheader20::NUMX),
//             );
//             base_offset
//                 + NitfImageSubheader20::get_value(NitfImageSubheader20::NUMX)
//                 + (num_text_segments
//                     * (NitfImageSubheader20::get_value(NitfImageSubheader20::LTSHNNN)
//                         + NitfImageSubheader20::get_value(NitfImageSubheader20::LTNNN)))
//         }
//         NitfImageSubheader20::NUMDES => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of DESegments."
//                 ),
//             }
//             let header_length = read_int_from_file(
//                 &file,
//                 NitfImageSubheader20::get_offset(NitfImageSubheader20::HL, None),
//                 NitfImageSubheader20::get_value(NitfImageSubheader20::HL),
//             );
//             header_length
//                 - (NitfImageSubheader20::get_value(NitfImageSubheader20::XHDL)
//                     + NitfImageSubheader20::get_value(NitfImageSubheader20::UDHDL)
//                     + NitfImageSubheader20::get_value(NitfImageSubheader20::NUMRES)
//                     + NitfImageSubheader20::get_value(NitfImageSubheader20::NUMDES))
//         }
//         NitfImageSubheader20::NUMRES => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of segments."
//                 ),
//             }
//             let header_length = read_int_from_file(
//                 &file,
//                 NitfImageSubheader20::get_offset(NitfImageSubheader20::HL, None),
//                 NitfImageSubheader20::get_value(NitfImageSubheader20::HL),
//             );
//             header_length
//                 - (NitfImageSubheader20::get_value(NitfImageSubheader20::XHDL)
//                     + NitfImageSubheader20::get_value(NitfImageSubheader20::UDHDL)
//                     + NitfImageSubheader20::get_value(NitfImageSubheader20::NUMRES))
//         }
//         NitfImageSubheader20::UDHDL => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of segments."
//                 ),
//             }
//             let header_length = read_int_from_file(
//                 &file,
//                 NitfImageSubheader20::get_offset(NitfImageSubheader20::HL, None),
//                 NitfImageSubheader20::get_value(NitfImageSubheader20::HL),
//             );
//             header_length
//                 - (NitfImageSubheader20::get_value(NitfImageSubheader20::XHDL)
//                     + NitfImageSubheader20::get_value(NitfImageSubheader20::UDHDL))
//         }
//         NitfImageSubheader20::XHDL => {
//             let file;
//             match nitf_file {
//                 Some(f) => file = f,
//                 None => panic!(
//                     "You must provide a file to get the offset of the number of segments."
//                 ),
//             }
//             let header_length = read_int_from_file(
//                 &file,
//                 NitfImageSubheader20::get_offset(NitfImageSubheader20::HL, None),
//                 NitfImageSubheader20::get_value(NitfImageSubheader20::HL),
//             );
//             header_length - NitfImageSubheader20::get_value(NitfImageSubheader20::XHDL)
//         }
//         NitfImageSubheader20::LISHNNN
//         | NitfImageSubheader20::LINNN
//         | NitfImageSubheader20::LSSHNNN
//         | NitfImageSubheader20::LSNNN
//         | NitfImageSubheader20::LTSHNNN
//         | NitfImageSubheader20::LTNNN => {
//             eprint!("You cannot get the single offset of N number of segments. Consider finding the offset of the number of segments and adding the size of the Subheader and Data length fields you are looking as many times as you need to get to the header offset you are looking for.");
//             0
//         }
//         _ => {
//             let index = target as usize;
//             NitfImageSubheader20::values()[..index].iter().sum()
//         }
//     }
//}
//}

