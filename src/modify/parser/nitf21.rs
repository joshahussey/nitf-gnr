use crate::modify::parser::file_ops::{
    read_int_from_bytes, read_int_from_file, read_string_from_file,
};
use std::fs::File;
use std::str::FromStr;

const FHDR: usize = 4;
const FVER: usize = 5;

pub enum NitfVersion {
    V01_10,
    V02_00,
    V02_10,
}

impl NitfVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            NitfVersion::V01_10 => "01.10",
            NitfVersion::V02_00 => "02.00",
            NitfVersion::V02_10 => "02.10",
        }
    }
}

impl FromStr for NitfVersion {
    type Err = String; // Define the error type (you can also use a custom error type here)

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        match version {
            "01.10" => Ok(NitfVersion::V01_10),
            "02.00" => Ok(NitfVersion::V02_00),
            "02.10" => Ok(NitfVersion::V02_10),
            _ => Err(format!("Invalid NITF version: {}", version)), // Return an error if the string doesn’t match
        }
    }
}

#[derive(Default, Debug)]
pub struct Header {
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
    pub fscop: usize,
    pub fscpys: usize,
    pub encryp: usize,
    pub fbkgc: usize,
    pub oname: usize,
    pub ophone: usize,
    pub fl: usize,
    pub hl: usize,
    pub numi: usize,
    pub lish: usize,
    pub li: usize,
    pub nums: usize,
    pub lssh: usize,
    pub ls: usize,
    pub numx: usize,
    pub numt: usize,
    pub ltsh: usize,
    pub lt: usize,
    pub numdes: usize,
    pub ldsh: usize,
    pub ldnnn: usize,
    pub numres: usize,
    pub lresh: usize,
    pub lre: usize,
    pub udhdl: usize,
    pub udhofl: usize,
    pub udhd: usize,
    pub xhdl: usize,
    pub xhdlofl: usize,
    pub xhd: usize,
}

#[derive(Default, Debug)]
pub struct Segments {
    pub image_segments: usize,
    pub graphic_segments: usize,
    pub reserved_segments: usize,
    pub text_segments: usize,
    pub data_extension_segments: usize,
    pub reserved_extension_segments: usize,
}

#[derive(Default, Debug)]
pub struct Nitf {
    pub header: Header,
    pub segments: Segments,
}

impl Nitf {
    pub fn get_version(file: &File) -> NitfVersion {
        NitfVersion::from_str(&read_string_from_file(file, FHDR, FVER)).unwrap()
    }

    pub fn new(file: &std::fs::File) -> Nitf {
        #[cfg(all(debug_assertions, not(test)))]
        println!("Loading NITF file.");
        let ver = Nitf::get_version(file);
        #[cfg(all(debug_assertions, not(test)))]
        println!("NITF Version: {}", ver.as_str());
        match ver {
            NitfVersion::V01_10 => panic!("Version 01.10 is not supported."),
            NitfVersion::V02_00 => panic!("Version 02.00 is not supported."),
            NitfVersion::V02_10 => Nitf::load_v02_10(file),
        }
    }

    pub fn load_v02_10(file: &std::fs::File) -> Nitf {
        use NitfHeader21::*;
        #[cfg(all(debug_assertions, not(test)))]
        println!("Loading NITF 02.10 file.");
        let mut nitf = Nitf {
            header: Header {
                fhdr: NitfHeader21::get_offset(FHDR, None),
                fver: NitfHeader21::get_offset(FVER, None),
                clevel: NitfHeader21::get_offset(CLEVEL, None),
                stype: NitfHeader21::get_offset(STYPE, None),
                ostaid: NitfHeader21::get_offset(OSTAID, None),
                fdt: NitfHeader21::get_offset(FDT, None),
                ftitle: NitfHeader21::get_offset(FTITLE, None),
                fsclas: NitfHeader21::get_offset(FSCLAS, None),
                fsclsy: NitfHeader21::get_offset(FSCLSY, None),
                fscode: NitfHeader21::get_offset(FSCODE, None),
                fsctlh: NitfHeader21::get_offset(FSCTLH, None),
                fsrel: NitfHeader21::get_offset(FSREL, None),
                fsdctp: NitfHeader21::get_offset(FSDCTP, None),
                fsdcdt: NitfHeader21::get_offset(FSDCDT, None),
                fsdcxm: NitfHeader21::get_offset(FSDCXM, None),
                fsdg: NitfHeader21::get_offset(FSDG, None),
                fsdgdt: NitfHeader21::get_offset(FSDGDT, None),
                fscltx: NitfHeader21::get_offset(FSCLTX, None),
                fscatp: NitfHeader21::get_offset(FSCATP, None),
                fscaut: NitfHeader21::get_offset(FSCAUT, None),
                fscrsn: NitfHeader21::get_offset(FSCRSN, None),
                fssrdt: NitfHeader21::get_offset(FSSRDT, None),
                fsctln: NitfHeader21::get_offset(FSCTLN, None),
                fscop: NitfHeader21::get_offset(FSCOP, None),
                fscpys: NitfHeader21::get_offset(FSCPYS, None),
                encryp: NitfHeader21::get_offset(ENCRYP, None),
                fbkgc: NitfHeader21::get_offset(FBKGC, None),
                oname: NitfHeader21::get_offset(ONAME, None),
                ophone: NitfHeader21::get_offset(OPHONE, None),
                fl: NitfHeader21::get_offset(FL, None),
                hl: NitfHeader21::get_offset(HL, None),
                numi: NitfHeader21::get_offset(NUMI, None),
                lish: NitfHeader21::get_offset(LISH, Some(file)),
                li: NitfHeader21::get_offset(LI, Some(file)),
                nums: NitfHeader21::get_offset(NUMS, Some(file)),
                lssh: NitfHeader21::get_offset(LSSH, Some(file)),
                ls: NitfHeader21::get_offset(LS, Some(file)),
                numx: NitfHeader21::get_offset(NUMX, Some(file)),
                numt: NitfHeader21::get_offset(NUMT, Some(file)),
                ltsh: NitfHeader21::get_offset(LTSH, Some(file)),
                lt: NitfHeader21::get_offset(LT, Some(file)),
                numdes: NitfHeader21::get_offset(NUMDES, Some(file)),
                ldsh: NitfHeader21::get_offset(LDSH, Some(file)),
                ldnnn: NitfHeader21::get_offset(LD, Some(file)),
                numres: NitfHeader21::get_offset(NUMRES, Some(file)),
                lresh: NitfHeader21::get_offset(LRESH, Some(file)),
                lre: NitfHeader21::get_offset(LRE, Some(file)),
                udhdl: 0,
                // udhdl: NitfHeader21::get_offset(UDHDL, Some(file)),
                udhofl: 0,
                //udhofl: NitfHeader21::get_offset(UDHOFL, Some(file)),
                udhd: 0,
                //udhd: NitfHeader21::get_offset(UDHD, Some(file)),
                xhdl: 0,
                // xhdl: NitfHeader21::get_offset(XHDL, Some(file)),
                xhdlofl: 0,
                // xhdlofl: NitfHeader21::get_offset(XHDLOFL, Some(file)),
                xhd: 0, //xhd: NitfHeader21::get_offset(XHD, Some(file)),
            },
            segments: Segments {
                image_segments: 0,
                graphic_segments: 0,
                reserved_segments: 0,
                text_segments: 0,
                data_extension_segments: 0,
                reserved_extension_segments: 0,
            },
        };
        #[cfg(all(debug_assertions, not(test)))]
        println!("Succesfully retrieved header offsets.");
        nitf.segments.image_segments = nitf.get_image_segments_offset(file);
        nitf.segments.graphic_segments = nitf.get_graphics_segments_offset(file);
        nitf.segments.reserved_segments = nitf.get_reserved_extension_segments_offset(file);
        nitf.segments.text_segments = nitf.get_text_segments_offset(file);
        nitf.segments.data_extension_segments = nitf.get_data_extension_segments_offset(file);
        nitf.segments.reserved_extension_segments =
            nitf.get_reserved_extension_segments_offset(file);
        #[cfg(all(debug_assertions, not(test)))]
        {
            println!("************************************************");
            println!("Image Segments: {}", nitf.segments.image_segments);
            println!("Graphic Segments: {}", nitf.segments.graphic_segments);
            println!("Reserved Segments: {}", nitf.segments.reserved_segments);
            println!("Text Segments: {}", nitf.segments.text_segments);
            println!(
                "Data Extension Segments: {}",
                nitf.segments.data_extension_segments
            );
            println!(
                "Reserved Extension Segments: {}",
                nitf.segments.reserved_extension_segments
            );
            println!(
                "nitf FL: {}",
                read_int_from_file(file, nitf.header.fl, NitfHeader21::get_value(FL))
            );
            println!("************************************************");
        }
        nitf
    }

    pub fn get_image_headers_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let numi = read_int_from_file(file, self.header.numi, N::get_value(NUMI));
        numi * (N::get_value(LISH) + N::get_value(LI))
    }

    pub fn get_graphic_headers_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let nums = read_int_from_file(file, self.header.nums, N::get_value(NUMS));
        nums * (N::get_value(LSSH) + N::get_value(LS))
    }

    pub fn get_text_headers_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let numt = read_int_from_file(file, self.header.numt, N::get_value(NUMT));
        numt * (N::get_value(LTSH) + N::get_value(LT))
    }

    pub fn get_data_extension_headers_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let numdes = read_int_from_file(file, self.header.numdes, N::get_value(NUMDES));
        numdes * (N::get_value(LDSH) + N::get_value(LD))
    }

    pub fn get_reserved_extension_headers_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let numres = read_int_from_file(file, self.header.numres, N::get_value(NUMRES));
        numres * (N::get_value(LRESH) + N::get_value(LRE))
    }

    pub fn get_image_seg_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let mut image_segment_length = 0;
        let mut header_cursor = self.header.lish;
        for _ in 0..read_int_from_file(file, self.header.numi, N::get_value(NUMI)) {
            image_segment_length += read_int_from_file(file, header_cursor, N::get_value(LISH));
            header_cursor += N::get_value(LISH);
            image_segment_length += read_int_from_file(file, header_cursor, N::get_value(LI));
            header_cursor += N::get_value(LI);
        }
        image_segment_length
    }

    pub fn get_graphic_seg_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let mut graphic_segment_length = 0;
        let mut header_cursor = self.header.lssh;
        for _ in 0..read_int_from_file(file, self.header.nums, N::get_value(NUMS)) {
            graphic_segment_length +=
                read_int_from_file(file, header_cursor, N::get_value(LSSH));
            header_cursor += N::get_value(LSSH);
            graphic_segment_length += read_int_from_file(file, header_cursor, N::get_value(LS));
            header_cursor += N::get_value(LS);
        }
        graphic_segment_length
    }

    pub fn get_text_seg_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let mut text_segment_length = 0;
        let mut header_cursor = N::get_offset(LTSH, Some(file));
        for _ in 0..read_int_from_file(file, self.header.numt, N::get_value(NUMT)) {
            text_segment_length += read_int_from_file(file, header_cursor, N::get_value(LTSH));
            header_cursor += N::get_value(LTSH);
            text_segment_length += read_int_from_file(file, header_cursor, N::get_value(LT));
            header_cursor += N::get_value(LT);
        }
        text_segment_length
    }

    pub fn get_data_extension_seg_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let mut data_extension_segment_length = 0;
        let mut header_cursor = self.header.ldsh;
        for _ in 0..read_int_from_file(file, self.header.numdes, N::get_value(NUMDES)) {
            data_extension_segment_length +=
                read_int_from_file(file, header_cursor, N::get_value(LDSH));
            header_cursor += N::get_value(LDSH);
            data_extension_segment_length +=
                read_int_from_file(file, header_cursor, N::get_value(LD));
            header_cursor += N::get_value(LD);
        }
        data_extension_segment_length
    }

    pub fn get_reserved_extension_seg_length(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let mut reserved_extension_segment_length = 0;
        let mut header_cursor = self.header.lresh;
        for _ in 0..read_int_from_file(file, self.header.numres, N::get_value(NUMRES)) {
            reserved_extension_segment_length +=
                read_int_from_file(file, header_cursor, N::get_value(LRESH));
            header_cursor += N::get_value(LRESH);
            reserved_extension_segment_length +=
                read_int_from_file(file, header_cursor, N::get_value(LRE));
            header_cursor += N::get_value(LRE);
        }
        reserved_extension_segment_length
    }

    pub fn get_image_segments_offset(&self, file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        read_int_from_file(file, N::get_offset(HL, Some(file)), N::get_value(HL))
    }

    pub fn get_graphics_segments_offset(&self, file: &File) -> usize {
        let image_seg_offset = self.get_image_segments_offset(file);
        let image_segment_length = self.get_image_seg_length(file);
        image_seg_offset + image_segment_length
    }

    pub fn get_text_segments_offset(&self, file: &File) -> usize {
        let graphic_seg_offset = self.get_graphics_segments_offset(file);
        let graphic_segment_length = self.get_graphic_seg_length(file);
        graphic_seg_offset + graphic_segment_length
    }

    pub fn get_data_extension_segments_offset(&self, file: &File) -> usize {
        let text_seg_offset = self.get_text_segments_offset(file);
        let text_segment_length = self.get_text_seg_length(file);
        text_seg_offset + text_segment_length
    }

    pub fn get_reserved_extension_segments_offset(&self, file: &File) -> usize {
        let data_ext_seg_offset = self.get_data_extension_segments_offset(file);
        let data_ext_segment_length = self.get_data_extension_seg_length(file);
        data_ext_seg_offset + data_ext_segment_length
    }
}

#[derive(Debug, Copy, Clone)]
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
    LISH,
    LI,
    NUMS,
    LSSH,
    LS,
    NUMX,
    NUMT,
    LTSH,
    LT,
    NUMDES,
    LDSH,
    LD,
    NUMRES,
    LRESH,
    LRE,
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
            3, 24, 18, 12, 6, 3, 6, 10, 3, 4, 6, 3, 3, 4, 5, 3, 4, 9, 3, 4, 7, 5, 3, 0, 5, 3, 0,
        ]
    }
    pub fn as_str(&self) -> &'static str {
        //As str
        match self {
            NitfHeader21::FHDR => "FHDR",
            NitfHeader21::FVER => "FVER",
            NitfHeader21::CLEVEL => "CLEVEL",
            NitfHeader21::STYPE => "STYPE",
            NitfHeader21::OSTAID => "OSTAID",
            NitfHeader21::FDT => "FDT",
            NitfHeader21::FTITLE => "FTITLE",
            NitfHeader21::FSCLAS => "FSCLAS",
            NitfHeader21::FSCLSY => "FSCLSY",
            NitfHeader21::FSCODE => "FSCODE",
            NitfHeader21::FSCTLH => "FSCTLH",
            NitfHeader21::FSREL => "FSREL",
            NitfHeader21::FSDCTP => "FSDCTP",
            NitfHeader21::FSDCDT => "FSDCDT",
            NitfHeader21::FSDCXM => "FSDCXM",
            NitfHeader21::FSDG => "FSDG",
            NitfHeader21::FSDGDT => "FSDGDT",
            NitfHeader21::FSCLTX => "FSCLTX",
            NitfHeader21::FSCATP => "FSCATP",
            NitfHeader21::FSCAUT => "FSCAUT",
            NitfHeader21::FSCRSN => "FSCRSN",
            NitfHeader21::FSSRDT => "FSSRDT",
            NitfHeader21::FSCTLN => "FSCTLN",
            NitfHeader21::FSCOP => "FSCOP",
            NitfHeader21::FSCPYS => "FSCPYS",
            NitfHeader21::ENCRYP => "ENCRYP",
            NitfHeader21::FBKGC => "FBKGC",
            NitfHeader21::ONAME => "ONAME",
            NitfHeader21::OPHONE => "OPHONE",
            NitfHeader21::FL => "FL",
            NitfHeader21::HL => "HL",
            NitfHeader21::NUMI => "NUMI",
            NitfHeader21::LISH => "LISHNNN",
            NitfHeader21::LI => "LINNN",
            NitfHeader21::NUMS => "NUMS",
            NitfHeader21::LSSH => "LSSHNNN",
            NitfHeader21::LS => "LSNNN",
            NitfHeader21::NUMX => "NUMX",
            NitfHeader21::NUMT => "NUMT",
            NitfHeader21::LTSH => "LTSHNNN",
            NitfHeader21::LT => "LTNNN",
            NitfHeader21::NUMDES => "NUMDES",
            NitfHeader21::LDSH => "LDSHNNN",
            NitfHeader21::LD => "LDNNN",
            NitfHeader21::NUMRES => "NUMRES",
            NitfHeader21::LRESH => "LRESHNNN",
            NitfHeader21::LRE => "LRENNN",
            NitfHeader21::UDHDL => "UDHDL",
            NitfHeader21::UDHOFL => "UDHOFL",
            NitfHeader21::UDHD => "UDHD",
            NitfHeader21::XHDL => "XHDL",
            NitfHeader21::XHDLOFL => "XHDLOFL",
            NitfHeader21::XHD => "XHD",
        }
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
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of NUMS header field.");
                }
                let base_offset = N::get_offset(NUMI, None);
                let num_images = read_int_from_file(
                    nitf_file.expect("Dev Error, this should not be called with no file."),
                    base_offset,
                    N::get_value(NUMI),
                );
                base_offset
                    + N::get_value(NUMI)
                    + (num_images * (N::get_value(LISH) + N::get_value(LI)))
            }
            LSSH => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of LSSHNNN header field.");
                }
                N::get_offset(NUMS, nitf_file) + N::get_value(NUMS)
            }
            LS => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of LSNNN header field.");
                }
                N::get_offset(LSSH, nitf_file) + N::get_value(LSSH)
            }
            //NUMX IS RESERVED FOR FUTURE USE
            NUMX => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of NUMX header field.");
                }
                let lsshnnn_offset = N::get_offset(LSSH, nitf_file);
                let num_segments = read_int_from_file(
                    nitf_file
                        .expect("You must provide a file to get the offset of NUMX header field."),
                    N::get_offset(NUMS, nitf_file),
                    N::get_value(NUMS),
                );
                lsshnnn_offset + (num_segments * (N::get_value(LSSH) + N::get_value(LS)))
            }
            //Text Segment offsets
            NUMT => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of NUMT header field.");
                }
                let numx_offset = N::get_offset(NUMX, nitf_file);
                numx_offset + N::get_value(NUMX)
            }
            LTSH => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of LTSHNNN header field.");
                }
                N::get_offset(NUMT, nitf_file) + N::get_value(NUMT)
            }
            LT => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of LTNNN header field.");
                }
                N::get_offset(LTSH, nitf_file) + N::get_value(LTSH)
            }
            //Data Extension Segment offsets
            NUMDES => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of NUMDES header field.");
                }
                let num_text = read_int_from_file(
                    nitf_file.expect(
                        "You must provide a file to get the offset of NUMDES header field.",
                    ),
                    N::get_offset(NUMT, nitf_file),
                    N::get_value(NUMT),
                );
                N::get_offset(LTSH, nitf_file)
                    + (num_text * (N::get_value(LTSH) + N::get_value(LT)))
            }
            LDSH => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of LDSHNNN header field.");
                }
                N::get_offset(NUMDES, nitf_file) + N::get_value(NUMDES)
            }
            LD => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of LDNNN header field.");
                }
                N::get_offset(LDSH, nitf_file) + N::get_value(LDSH)
            }
            //Reserved Extension Segment offsets
            NUMRES => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of NUMRES header field.");
                }
                let num_des = read_int_from_file(
                    nitf_file.expect(
                        "You must provide a file to get the offset of NUMRES header field.",
                    ),
                    N::get_offset(NUMDES, nitf_file),
                    N::get_value(NUMDES),
                );
                N::get_offset(LDSH, nitf_file)
                    + (num_des * (N::get_value(LDSH) + N::get_value(LD)))
            }
            LRESH => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of LRESHNNN header field.");
                }
                N::get_offset(NUMRES, nitf_file) + N::get_value(NUMRES)
            }
            LRE => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of LRENNN header field.");
                }
                N::get_offset(LRESH, nitf_file) + N::get_value(LRESH)
            }
            //User Defined Header offsets
            UDHDL => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let num_res = read_int_from_file(
                    nitf_file
                        .expect("You must provide a file to get the offset of this header field."),
                    N::get_offset(NUMRES, nitf_file),
                    N::get_value(NUMRES),
                );
                N::get_offset(LRESH, nitf_file)
                    + (num_res * (N::get_value(LRESH) + N::get_value(LRE)))
            }
            UDHOFL => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset(UDHDL, nitf_file) + N::get_value(UDHDL)
            }
            UDHD => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset(UDHOFL, nitf_file) + N::get_value(UDHOFL)
            }
            //Extended Header Data Length Segment offsets
            XHDL => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let udhd_len = read_int_from_file(
                    nitf_file
                        .expect("You must provide a file to get the offset of this header field."),
                    N::get_offset(UDHDL, nitf_file),
                    N::get_value(UDHDL),
                );
                N::get_offset(UDHOFL, nitf_file) + udhd_len
            }
            XHDLOFL => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset(XHDL, nitf_file) + N::get_value(XHDL)
            }
            XHD => {
                if nitf_file.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let xhd_len = read_int_from_file(
                    nitf_file
                        .expect("You must provide a file to get the offset of this header field."),
                    N::get_offset(XHDL, nitf_file),
                    N::get_value(XHDL),
                );
                N::get_offset(XHDLOFL, nitf_file) + xhd_len
            }
            //Anything before NUMS
            _ => {
                let index = target as usize;
                N::values()[..index].iter().sum()
            }
        }
    }

    pub fn change_field_value(
        target: NitfHeader21,
        file: &File,
        file_buf: &mut [u8],
        new_value: &[u8],
    ) {
        let off = NitfHeader21::get_offset(target, Some(file));
        let len = NitfHeader21::get_value(target);
        if new_value.len() != len {
            eprint!("New value not the same length as field... Are you sure you want to do this?");
        }
        file_buf[off..off + len].copy_from_slice(new_value);
    }

    pub fn get_offset_bytes(target: NitfHeader21, bytes: Option<&Vec<u8>>) -> usize {
        use NitfHeader21::{self as N, *};
        match target {
            //Graphic Segment offsets
            NUMS => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let base_offset = N::get_offset_bytes(NUMI, None);
                let num_images = read_int_from_bytes(
                    bytes.expect("Dev Error, this should not be called with no file."),
                    base_offset,
                    N::get_value(NUMI),
                );
                base_offset
                    + N::get_value(NUMI)
                    + (num_images * (N::get_value(LISH) + N::get_value(LI)))
            }
            LSSH => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(NUMS, bytes) + N::get_value(NUMS)
            }
            LS => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(LSSH, bytes) + N::get_value(LSSH)
            }
            //NUMX IS RESERVED FOR FUTURE USE
            NUMX => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let lsshnnn_offset = N::get_offset_bytes(LSSH, bytes);
                let num_segments = read_int_from_bytes(
                    bytes.expect("You must provide a file to get the offset of this header field."),
                    N::get_offset_bytes(NUMS, bytes),
                    N::get_value(NUMS),
                );
                lsshnnn_offset + (num_segments * (N::get_value(LSSH) + N::get_value(LS)))
            }
            //Text Segment offsets
            NUMT => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let numx_offset = N::get_offset_bytes(NUMX, bytes);
                numx_offset + N::get_value(NUMX)
            }
            LTSH => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(NUMT, bytes) + N::get_value(NUMT)
            }
            LT => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(LTSH, bytes) + N::get_value(LTSH)
            }
            //Data Extension Segment offsets
            NUMDES => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let num_text = read_int_from_bytes(
                    bytes.expect("You must provide a file to get the offset of this header field."),
                    N::get_offset_bytes(NUMT, bytes),
                    N::get_value(NUMT),
                );
                N::get_offset_bytes(LTSH, bytes)
                    + (num_text * (N::get_value(LTSH) + N::get_value(LT)))
            }
            LDSH => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(NUMDES, bytes) + N::get_value(NUMDES)
            }
            LD => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(LDSH, bytes) + N::get_value(LDSH)
            }
            //Reserved Extension Segment offsets
            NUMRES => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let num_des = read_int_from_bytes(
                    bytes.expect("You must provide a file to get the offset of this header field."),
                    N::get_offset_bytes(NUMDES, bytes),
                    N::get_value(NUMDES),
                );
                N::get_offset_bytes(NUMDES, bytes)
                    + (num_des * (N::get_value(LDSH) + N::get_value(LD)))
            }
            LRESH => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(NUMRES, bytes) + N::get_value(NUMRES)
            }
            LRE => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(LRESH, bytes) + N::get_value(LRESH)
            }
            //User Defined Header offsets
            UDHDL => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let num_res = read_int_from_bytes(
                    bytes.expect("You must provide a file to get the offset of this header field."),
                    N::get_offset_bytes(NUMRES, bytes),
                    N::get_value(NUMRES),
                );
                N::get_offset_bytes(NUMRES, bytes)
                    + (num_res * (N::get_value(LRESH) + N::get_value(LRE)))
            }
            UDHOFL => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(UDHDL, bytes) + N::get_value(UDHDL)
            }
            UDHD => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(UDHOFL, bytes) + N::get_value(UDHOFL)
            }
            //Extended Header Data Length Segment offsets
            XHDL => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let udhd_len = read_int_from_bytes(
                    bytes.expect("You must provide a file to get the offset of this header field."),
                    N::get_offset_bytes(UDHDL, bytes),
                    N::get_value(UDHDL),
                );
                N::get_offset_bytes(UDHOFL, bytes) + udhd_len
            }
            XHDLOFL => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                N::get_offset_bytes(XHDL, bytes) + N::get_value(XHDL)
            }
            XHD => {
                if bytes.is_none() {
                    panic!("You must provide a file to get the offset of this header field.");
                }
                let xhd_len = read_int_from_bytes(
                    bytes.expect("You must provide a file to get the offset of this header field."),
                    N::get_offset_bytes(XHDL, bytes),
                    N::get_value(XHDL),
                );
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
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images =
            read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let lish_nnn_offset = N::get_offset(LISH, nitf_file);
        lish_nnn_offset + (image_num as usize * (N::get_value(LISH) + N::get_value(LI)))
    }

    pub fn get_image_header_field_offset_bytes(
        nitf_bytes: Option<&Vec<u8>>,
        image_num: u64,
    ) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMI, nitf_bytes),
            N::get_value(NUMI),
        );
        if image_num + 1 > num_images as u64 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let lish_nnn_offset = N::get_offset_bytes(LISH, nitf_bytes);
        lish_nnn_offset + (image_num as usize * (N::get_value(LISH) + N::get_value(LI)))
    }

    pub fn get_image_data_field_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images =
            read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let lish_nnn_offset = N::get_image_header_field_offset(nitf_file, image_num);
        lish_nnn_offset + N::get_value(LISH)
    }

    pub fn get_image_data_field_offset_bytes(
        nitf_bytes: Option<&Vec<u8>>,
        image_num: u64,
    ) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMI, nitf_bytes),
            N::get_value(NUMI),
        );
        if image_num + 1 > num_images as u64 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let lish_nnn_offset = N::get_image_header_field_offset_bytes(nitf_bytes, image_num);
        lish_nnn_offset + N::get_value(LISH)
    }

    pub fn get_graphic_header_field_offset(nitf_file: Option<&File>, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_graphics =
            read_int_from_file(file, N::get_offset(NUMS, nitf_file), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let lssh_nnn_offset = N::get_offset(LSSH, nitf_file);
        lssh_nnn_offset + (graphic_num as usize * (N::get_value(LSSH) + N::get_value(LS)))
    }

    pub fn get_graphic_header_field_offset_bytes(
        nitf_bytes: Option<&Vec<u8>>,
        graphic_num: u64,
    ) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_graphics = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMS, nitf_bytes),
            N::get_value(NUMS),
        );
        if graphic_num + 1 > num_graphics as u64 {
            panic!("Graphic number is greater than the number of graphics in the bytes, they are 0 indexed, are you off by 1?");
        }
        let lssh_nnn_offset = N::get_offset_bytes(LSSH, nitf_bytes);
        lssh_nnn_offset + (graphic_num as usize * (N::get_value(LSSH) + N::get_value(LS)))
    }

    pub fn get_graphic_data_field_offset(nitf_file: Option<&File>, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_graphics =
            read_int_from_file(file, N::get_offset(NUMS, nitf_file), N::get_value(NUMS));
        if graphic_num + 1 > num_graphics as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let lssh_nnn_offset = N::get_graphic_header_field_offset(nitf_file, graphic_num);
        lssh_nnn_offset + N::get_value(LSSH)
    }

    pub fn get_graphic_data_field_offset_bytes(
        nitf_bytes: Option<&Vec<u8>>,
        graphic_num: u64,
    ) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_graphics = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMS, nitf_bytes),
            N::get_value(NUMS),
        );
        if graphic_num + 1 > num_graphics as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let lssh_nnn_offset = N::get_graphic_header_field_offset_bytes(nitf_bytes, graphic_num);
        lssh_nnn_offset + N::get_value(LSSH)
    }

    pub fn get_text_header_field_offset(nitf_file: Option<&File>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_file(file, N::get_offset(NUMT, nitf_file), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let ltsh_nnn_offset = N::get_offset(LTSH, nitf_file);
        ltsh_nnn_offset + (text_num as usize * (N::get_value(LTSH) + N::get_value(LT)))
    }

    pub fn get_text_header_field_offset_bytes(
        nitf_bytes: Option<&Vec<u8>>,
        text_num: u64,
    ) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMT, nitf_bytes),
            N::get_value(NUMT),
        );
        if text_num + 1 > num_text as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let ltsh_nnn_offset = N::get_offset_bytes(LTSH, nitf_bytes);
        ltsh_nnn_offset + (text_num as usize * (N::get_value(LTSH) + N::get_value(LT)))
    }

    pub fn get_text_data_field_offset(nitf_file: Option<&File>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_file(file, N::get_offset(NUMT, nitf_file), N::get_value(NUMT));
        if text_num + 1 > num_text as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let ltsh_nnn_offset = N::get_text_header_field_offset(nitf_file, text_num);
        ltsh_nnn_offset + N::get_value(LTSH)
    }

    pub fn get_text_data_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_text = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMT, nitf_bytes),
            N::get_value(NUMT),
        );
        if text_num + 1 > num_text as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let ltsh_nnn_offset = N::get_text_header_field_offset_bytes(nitf_bytes, text_num);
        ltsh_nnn_offset + N::get_value(LTSH)
    }

    /// Get the offset of the Data Extension Segment header length header field
    pub fn get_des_header_field_offset(nitf_file: Option<&File>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_des =
            read_int_from_file(file, N::get_offset(NUMDES, nitf_file), N::get_value(NUMDES));
        if des_num + 1 > num_des as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let ldsh_nnn_offset = N::get_offset(LDSH, nitf_file);
        ldsh_nnn_offset + (des_num as usize * (N::get_value(LDSH) + N::get_value(LD)))
    }

    pub fn get_des_header_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_des = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMDES, nitf_bytes),
            N::get_value(NUMDES),
        );
        if des_num + 1 > num_des as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let ldsh_nnn_offset = N::get_offset_bytes(LDSH, nitf_bytes);
        ldsh_nnn_offset + (des_num as usize * (N::get_value(LDSH) + N::get_value(LD)))
    }

    /// Get the offset of the Data Extension Segment data length header field
    pub fn get_des_data_field_offset(nitf_file: Option<&File>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_des =
            read_int_from_file(file, N::get_offset(NUMDES, nitf_file), N::get_value(NUMDES));
        if des_num + 1 > num_des as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let ldsh_nnn_offset = N::get_des_header_field_offset(nitf_file, des_num);
        ldsh_nnn_offset + N::get_value(LDSH)
    }

    pub fn get_des_data_field_offset_bytes(nitf_bytes: Option<&Vec<u8>>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_des = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMDES, nitf_bytes),
            N::get_value(NUMDES),
        );
        if des_num + 1 > num_des as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let ldsh_nnn_offset = N::get_des_header_field_offset_bytes(nitf_bytes, des_num);
        ldsh_nnn_offset + N::get_value(LDSH)
    }

    pub fn get_reserved_header_field_offset(nitf_file: Option<&File>, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_res =
            read_int_from_file(file, N::get_offset(NUMRES, nitf_file), N::get_value(NUMRES));
        if res_num + 1 > num_res as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let lresh_nnn_offset = N::get_offset(LRESH, nitf_file);
        lresh_nnn_offset + (res_num as usize * (N::get_value(LRESH) + N::get_value(LRE)))
    }

    pub fn get_reserved_header_field_offset_bytes(
        nitf_bytes: Option<&Vec<u8>>,
        res_num: u64,
    ) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_res = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMRES, nitf_bytes),
            N::get_value(NUMRES),
        );
        if res_num + 1 > num_res as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let lresh_nnn_offset = N::get_offset_bytes(LRESH, nitf_bytes);
        lresh_nnn_offset + (res_num as usize * (N::get_value(LRESH) + N::get_value(LRE)))
    }

    pub fn get_reserved_data_field_offset(nitf_file: Option<&File>, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_res =
            read_int_from_file(file, N::get_offset(NUMRES, nitf_file), N::get_value(NUMRES));
        if res_num + 1 > num_res as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let lresh_nnn_offset = N::get_reserved_header_field_offset(nitf_file, res_num);
        lresh_nnn_offset + N::get_value(LRESH)
    }

    pub fn get_reserved_data_field_offset_bytes(
        nitf_bytes: Option<&Vec<u8>>,
        res_num: u64,
    ) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_res = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMRES, nitf_bytes),
            N::get_value(NUMRES),
        );
        if res_num + 1 > num_res as u64 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let lresh_nnn_offset = N::get_reserved_header_field_offset_bytes(nitf_bytes, res_num);
        lresh_nnn_offset + N::get_value(LRESH)
    }

    pub fn get_image_subheader_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images =
            read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let header_length =
            read_int_from_file(file, N::get_offset(HL, nitf_file), N::get_value(HL));
        if image_num == 0 {
            return header_length;
        }
        let mut offset = header_length;
        for i in 0..image_num {
            offset += read_int_from_file(
                file,
                N::get_image_header_field_offset(nitf_file, i),
                N::get_value(LISH),
            ) + read_int_from_file(
                file,
                N::get_image_data_field_offset(nitf_file, i),
                N::get_value(LI),
            );
        }
        offset
    }

    pub fn get_image_subheader_offset_bytes(nitf_bytes: Option<&Vec<u8>>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMI, nitf_bytes),
            N::get_value(NUMI),
        );
        if image_num + 1 > num_images as u64 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let header_length =
            read_int_from_bytes(bytes, N::get_offset_bytes(HL, nitf_bytes), N::get_value(HL));
        if image_num == 0 {
            return header_length;
        }
        let mut offset = header_length;
        for i in 0..image_num {
            offset += read_int_from_bytes(
                bytes,
                N::get_image_header_field_offset_bytes(nitf_bytes, i),
                N::get_value(LISH),
            ) + read_int_from_bytes(
                bytes,
                N::get_image_data_field_offset_bytes(nitf_bytes, i),
                N::get_value(LI),
            );
        }
        offset
    }

    pub fn get_image_data_offset(nitf_file: Option<&File>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let file =
            nitf_file.expect("You must provide a file to get the offset of this header field.");
        let num_images =
            read_int_from_file(file, N::get_offset(NUMI, nitf_file), N::get_value(NUMI));
        if image_num + 1 > num_images as u64 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let image_subheader_length = read_int_from_file(
            file,
            N::get_image_subheader_offset(nitf_file, image_num),
            N::get_value(LISH),
        );
        N::get_image_subheader_offset(nitf_file, image_num) + image_subheader_length
    }

    pub fn get_image_data_offset_bytes(nitf_bytes: Option<&Vec<u8>>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let bytes =
            nitf_bytes.expect("You must provide a file to get the offset of this header field.");
        let num_images = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMI, nitf_bytes),
            N::get_value(NUMI),
        );
        if image_num + 1 > num_images as u64 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let image_subheader_length = read_int_from_bytes(
            bytes,
            N::get_image_subheader_offset_bytes(nitf_bytes, image_num),
            N::get_value(LISH),
        );
        N::get_image_subheader_offset_bytes(nitf_bytes, image_num) + image_subheader_length
    }

    pub fn get_images_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        read_int_from_file(
            nitf_file,
            N::get_offset(HL, Some(nitf_file)),
            N::get_value(HL),
        )
    }

    pub fn get_images_segment_start_bytes(bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        read_int_from_bytes(
            bytes,
            N::get_offset_bytes(HL, Some(bytes)),
            N::get_value(HL),
        )
    }

    pub fn get_graphic_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let num_images = read_int_from_file(
            nitf_file,
            N::get_offset(NUMI, Some(nitf_file)),
            N::get_value(NUMI),
        );
        let mut offset = N::get_images_segment_start(nitf_file);
        for i in 0..num_images {
            offset += read_int_from_file(
                nitf_file,
                N::get_image_header_field_offset(Some(nitf_file), i as u64),
                N::get_value(LISH),
            ) + read_int_from_file(
                nitf_file,
                N::get_image_data_field_offset(Some(nitf_file), i as u64),
                N::get_value(LI),
            );
        }
        offset
    }

    pub fn get_graphic_segment_start_bytes(bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        let num_images = read_int_from_bytes(
            bytes,
            N::get_offset_bytes(NUMI, Some(bytes)),
            N::get_value(NUMI),
        );
        let mut offset = N::get_images_segment_start_bytes(bytes);
        for i in 0..num_images {
            offset += read_int_from_bytes(
                bytes,
                N::get_image_header_field_offset_bytes(Some(bytes), i as u64),
                N::get_value(LISH),
            ) + read_int_from_bytes(
                bytes,
                N::get_image_data_field_offset_bytes(Some(bytes), i as u64),
                N::get_value(LI),
            );
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
        let num_graphics = read_int_from_file(
            nitf_file,
            N::get_offset(NUMS, Some(nitf_file)),
            N::get_value(NUMS),
        );
        let mut offset = N::get_graphic_segment_start(nitf_file);
        for i in 0..num_graphics {
            offset += read_int_from_file(
                nitf_file,
                N::get_image_header_field_offset(Some(nitf_file), i as u64),
                N::get_value(LISH),
            ) + read_int_from_file(
                nitf_file,
                N::get_image_data_field_offset(Some(nitf_file), i as u64),
                N::get_value(LI),
            );
        }
        offset
    }

    pub fn get_text_segment_start_bytes(nitf_bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        let num_graphics = read_int_from_bytes(
            nitf_bytes,
            N::get_offset_bytes(NUMS, Some(nitf_bytes)),
            N::get_value(NUMS),
        );
        let mut offset = N::get_graphic_segment_start_bytes(nitf_bytes);
        for i in 0..num_graphics {
            offset += read_int_from_bytes(
                nitf_bytes,
                N::get_image_header_field_offset_bytes(Some(nitf_bytes), i as u64),
                N::get_value(LISH),
            ) + read_int_from_bytes(
                nitf_bytes,
                N::get_image_data_field_offset_bytes(Some(nitf_bytes), i as u64),
                N::get_value(LI),
            );
        }
        offset
    }

    pub fn get_des_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let num_text = read_int_from_file(
            nitf_file,
            N::get_offset(NUMT, Some(nitf_file)),
            N::get_value(NUMT),
        );
        let mut offset = N::get_text_segment_start(nitf_file);
        for i in 0..num_text {
            offset += read_int_from_file(
                nitf_file,
                N::get_text_header_field_offset(Some(nitf_file), i as u64),
                N::get_value(LTSH),
            ) + read_int_from_file(
                nitf_file,
                N::get_text_data_field_offset(Some(nitf_file), i as u64),
                N::get_value(LT),
            );
        }
        offset
    }

    pub fn get_des_segment_start_bytes(nitf_bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        let num_text = read_int_from_bytes(
            nitf_bytes,
            N::get_offset_bytes(NUMT, Some(nitf_bytes)),
            N::get_value(NUMT),
        );
        let mut offset = N::get_text_segment_start_bytes(nitf_bytes);
        for i in 0..num_text {
            offset += read_int_from_bytes(
                nitf_bytes,
                N::get_text_header_field_offset_bytes(Some(nitf_bytes), i as u64),
                N::get_value(LTSH),
            ) + read_int_from_bytes(
                nitf_bytes,
                N::get_text_data_field_offset_bytes(Some(nitf_bytes), i as u64),
                N::get_value(LT),
            );
        }
        offset
    }

    pub fn get_reserved_extension_segment_start(nitf_file: &File) -> usize {
        use NitfHeader21::{self as N, *};
        let num_des = read_int_from_file(
            nitf_file,
            N::get_offset(NUMDES, Some(nitf_file)),
            N::get_value(NUMDES),
        );
        let mut offset = N::get_des_segment_start(nitf_file);
        for i in 0..num_des {
            offset += read_int_from_file(
                nitf_file,
                N::get_des_header_field_offset(Some(nitf_file), i as u64),
                N::get_value(LDSH),
            ) + read_int_from_file(
                nitf_file,
                N::get_des_data_field_offset(Some(nitf_file), i as u64),
                N::get_value(LD),
            );
        }
        offset
    }

    pub fn get_reserved_extension_segment_start_bytes(nitf_bytes: &Vec<u8>) -> usize {
        use NitfHeader21::{self as N, *};
        let num_des = read_int_from_bytes(
            nitf_bytes,
            N::get_offset_bytes(NUMDES, Some(nitf_bytes)),
            N::get_value(NUMDES),
        );
        let mut offset = N::get_des_segment_start_bytes(nitf_bytes);
        for i in 0..num_des {
            offset += read_int_from_bytes(
                nitf_bytes,
                N::get_des_header_field_offset_bytes(Some(nitf_bytes), i as u64),
                N::get_value(LDSH),
            ) + read_int_from_bytes(
                nitf_bytes,
                N::get_des_data_field_offset_bytes(Some(nitf_bytes), i as u64),
                N::get_value(LD),
            );
        }
        offset
    }

    pub fn get_image_segments_offset(nitf_file: &File, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_images = read_int_from_file(
            nitf_file,
            N::get_offset(NUMI, Some(nitf_file)),
            N::get_value(NUMI),
        );
        if image_num + 1 > num_images as u64 && image_num != 0 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_images_segment_start(nitf_file);
        for i in 0..image_num {
            offset += read_int_from_file(
                nitf_file,
                N::get_image_header_field_offset(Some(nitf_file), i),
                N::get_value(LISH),
            ) + read_int_from_file(
                nitf_file,
                N::get_image_data_field_offset(Some(nitf_file), i),
                N::get_value(LI),
            );
        }
        offset
    }

    pub fn get_image_segments_offset_bytes(nitf_bytes: &Vec<u8>, image_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_images = read_int_from_bytes(
            nitf_bytes,
            N::get_offset_bytes(NUMI, Some(nitf_bytes)),
            N::get_value(NUMI),
        );
        if image_num + 1 > num_images as u64 && image_num != 0 {
            panic!("Image number is greater than the number of images in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_images_segment_start_bytes(nitf_bytes);
        for i in 0..image_num {
            offset += read_int_from_bytes(
                nitf_bytes,
                N::get_image_header_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LISH),
            ) + read_int_from_bytes(
                nitf_bytes,
                N::get_image_data_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LI),
            );
        }
        offset
    }

    pub fn get_graphic_segments_offset(nitf_file: &File, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_graphics = read_int_from_file(
            nitf_file,
            N::get_offset(NUMS, Some(nitf_file)),
            N::get_value(NUMS),
        );
        if graphic_num + 1 > num_graphics as u64 && graphic_num != 0 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_graphic_segment_start(nitf_file);
        for i in 0..graphic_num {
            offset += read_int_from_file(
                nitf_file,
                N::get_graphic_header_field_offset(Some(nitf_file), i),
                N::get_value(LSSH),
            ) + read_int_from_file(
                nitf_file,
                N::get_graphic_data_field_offset(Some(nitf_file), i),
                N::get_value(LS),
            );
        }
        offset
    }

    pub fn get_graphic_segments_offset_bytes(nitf_bytes: &Vec<u8>, graphic_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_graphics = read_int_from_bytes(
            nitf_bytes,
            N::get_offset_bytes(NUMS, Some(nitf_bytes)),
            N::get_value(NUMS),
        );
        if graphic_num + 1 > num_graphics as u64 && graphic_num != 0 {
            panic!("Graphic number is greater than the number of graphics in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_graphic_segment_start_bytes(nitf_bytes);
        for i in 0..graphic_num {
            offset += read_int_from_bytes(
                nitf_bytes,
                N::get_graphic_header_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LSSH),
            ) + read_int_from_bytes(
                nitf_bytes,
                N::get_graphic_data_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LS),
            );
        }
        offset
    }

    pub fn get_text_segments_offset(nitf_file: &File, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_text = read_int_from_file(
            nitf_file,
            N::get_offset(NUMT, Some(nitf_file)),
            N::get_value(NUMT),
        );
        if text_num + 1 > num_text as u64 && text_num != 0 {
            panic!("Text number is greater than the number of text segments in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_offset(NUMT, Some(nitf_file)) + N::get_value(NUMT);
        for i in 0..text_num {
            offset += read_int_from_file(
                nitf_file,
                N::get_text_header_field_offset(Some(nitf_file), i),
                N::get_value(LTSH),
            ) + read_int_from_file(
                nitf_file,
                N::get_text_data_field_offset(Some(nitf_file), i),
                N::get_value(LT),
            );
        }
        offset
    }

    pub fn get_text_segments_offset_bytes(nitf_bytes: &Vec<u8>, text_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_text = read_int_from_bytes(
            nitf_bytes,
            N::get_offset_bytes(NUMT, Some(nitf_bytes)),
            N::get_value(NUMT),
        );
        if text_num + 1 > num_text as u64 && text_num != 0 {
            panic!("Text number is greater than the number of text segments in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_offset_bytes(NUMT, Some(nitf_bytes)) + N::get_value(NUMT);
        for i in 0..text_num {
            offset += read_int_from_bytes(
                nitf_bytes,
                N::get_text_header_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LTSH),
            ) + read_int_from_bytes(
                nitf_bytes,
                N::get_text_data_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LT),
            );
        }
        offset
    }

    pub fn get_des_segments_offset(nitf_file: &File, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_des = read_int_from_file(
            nitf_file,
            N::get_offset(NUMDES, Some(nitf_file)),
            N::get_value(NUMDES),
        );
        if des_num + 1 > num_des as u64 && des_num != 0 {
            panic!("Data Extension number is greater than the number of data extension segments in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_des_segment_start(nitf_file);
        for i in 0..des_num {
            offset += read_int_from_file(
                nitf_file,
                N::get_des_header_field_offset(Some(nitf_file), i),
                N::get_value(LDSH),
            ) + read_int_from_file(
                nitf_file,
                N::get_des_data_field_offset(Some(nitf_file), i),
                N::get_value(LD),
            );
        }
        offset
    }

    pub fn get_des_segments_offset_bytes(nitf_bytes: &Vec<u8>, des_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_des = read_int_from_bytes(
            nitf_bytes,
            N::get_offset_bytes(NUMDES, Some(nitf_bytes)),
            N::get_value(NUMDES),
        );
        if des_num + 1 > num_des as u64 && des_num != 0 {
            panic!("Data Extension number is greater than the number of data extension segments in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_des_segment_start_bytes(nitf_bytes);
        for i in 0..des_num {
            offset += read_int_from_bytes(
                nitf_bytes,
                N::get_des_header_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LDSH),
            ) + read_int_from_bytes(
                nitf_bytes,
                N::get_des_data_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LD),
            );
        }
        offset
    }

    pub fn get_reserved_segments_offset(nitf_file: &File, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_res = read_int_from_file(
            nitf_file,
            N::get_offset(NUMRES, Some(nitf_file)),
            N::get_value(NUMRES),
        );
        if res_num + 1 > num_res as u64 && res_num != 0 {
            panic!("Reserved Extension number is greater than the number of reserved extension segments in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_reserved_extension_segment_start(nitf_file);
        for i in 0..res_num {
            offset += read_int_from_file(
                nitf_file,
                N::get_reserved_header_field_offset(Some(nitf_file), i),
                N::get_value(LRESH),
            ) + read_int_from_file(
                nitf_file,
                N::get_reserved_data_field_offset(Some(nitf_file), i),
                N::get_value(LRE),
            );
        }
        offset
    }

    pub fn get_reserved_segments_offset_bytes(nitf_bytes: &Vec<u8>, res_num: u64) -> usize {
        use NitfHeader21::{self as N, *};
        let num_res = read_int_from_bytes(
            nitf_bytes,
            N::get_offset_bytes(NUMRES, Some(nitf_bytes)),
            N::get_value(NUMRES),
        );
        if res_num + 1 > num_res as u64 && res_num != 0 {
            panic!("Reserved Extension number is greater than the number of reserved extension segments in the file, they are 0 indexed, are you off by 1?");
        }
        let mut offset = N::get_reserved_extension_segment_start_bytes(nitf_bytes);
        for i in 0..res_num {
            offset += read_int_from_bytes(
                nitf_bytes,
                N::get_reserved_header_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LRESH),
            ) + read_int_from_bytes(
                nitf_bytes,
                N::get_reserved_data_field_offset_bytes(Some(nitf_bytes), i),
                N::get_value(LRE),
            );
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
