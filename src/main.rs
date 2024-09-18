use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};


const FHDR: usize = 4;
const FVER: usize = 5;


#[derive(Default, Debug)]
struct NITF {
    // header values
    FHDR:     usize,
    FVER:     usize,
    CLEVEL:   usize,
    STYPE:    usize,
    OSTAID:   usize,
    FDT:      usize,
    FTITLE:   usize,
    FSCLAS:   usize,
    FSCLSY:   usize,
    FSCODE:   usize,
    FSCTLH:   usize,
    FSREL:    usize,
    FSDCTP:   usize,
    FSDCDT:   usize,
    FSDCXM:   usize,
    FSDG:     usize,
    FSDGDT:   usize,
    FSCLTX:   usize,
    FSCATP:   usize,
    FSCAUT:   usize,
    FSCRSN:   usize,
    FSSRDT:   usize,
    FSCTLN:   usize,
    FSDWNG:   usize,
    FSDEVT:   usize,
    FSCOP:    usize,
    FSCPYS:   usize,
    ENCRYP:   usize,
    FBKGC:    usize,
    ONAME:    usize,
    OPHONE:   usize,
    FL:       usize,
    HL:       usize,
    NUMI:     usize,
    LISH_NNN: usize,
    LI_NNN:   usize,
    // non-header values
    file_profile_name: String,
    file_version: String,
    header_length: usize,
    num_images: usize,
    image_header_lengths: Vec<usize>,
    image_data_lengths: Vec<usize>
}


impl NITF {
    fn get_file_profile_name_and_version(self, file: &File) -> NITF {
        let file_profile_name = read_string_from_file(&file, FHDR);
        let file_version = read_string_from_file(&file, FVER);

        NITF {
            file_profile_name: file_profile_name,
            file_version: file_version,
            ..self
        }
    }

    fn load_v02_10(self) -> NITF {
        NITF {
            FHDR:      4,
            FVER:      5,
            CLEVEL:    2,
            STYPE:     4,
            OSTAID:   10,
            FDT:      14,
            FTITLE:   80,
            FSCLAS:    1,
            FSCLSY:    2,
            FSCODE:   11,
            FSCTLH:    2,
            FSREL:    20,
            FSDCTP:    2,
            FSDCDT:    8,
            FSDCXM:    4,
            FSDG:      1,
            FSDGDT:    8,
            FSCLTX:   43,
            FSCATP:    1,
            FSCAUT:   40,
            FSCRSN:    1,
            FSSRDT:    8,
            FSCTLN:   15,
            FSCOP:     5,
            FSCPYS:    5,
            ENCRYP:    1,
            FBKGC:     3,
            ONAME:    24,
            OPHONE:   18,
            FL:       12,
            HL:        6,
            NUMI:      3,
            LISH_NNN:  6,
            LI_NNN:   10,
            ..self
        }
    }

    fn load_v02_00(self) -> NITF {
        NITF {
            FHDR:      9,
            CLEVEL:    2,
            STYPE:     4,
            OSTAID:   10,
            FDT:      14,
            FTITLE:   80,
            FSCLAS:    1,
            FSCODE:   40,
            FSCTLH:   40,
            FSREL:    40,
            FSCAUT:   20,
            FSCTLN:   20,
            FSDWNG:    6,
            FSDEVT:   40,
            FSCOP:     5,
            FSCPYS:    5,
            ENCRYP:    1,
            ONAME:    27,
            OPHONE:   18,
            FL:       12,
            HL:        6,
            NUMI:      3,
            LISH_NNN:  6,
            LI_NNN:   10,
            ..self
        }
    }

    // not implemented
    fn load_v01_10(self) -> NITF {
        NITF {
            FHDR:     0,
            FVER:     0,
            CLEVEL:   0,
            STYPE:    0,
            OSTAID:   0,
            FDT:      0,
            FTITLE:   0,
            FSCLAS:   0,
            FSCLSY:   0,
            FSCODE:   0,
            FSCTLH:   0,
            FSREL:    0,
            FSDCTP:   0,
            FSDCDT:   0,
            FSDCXM:   0,
            FSDG:     0,
            FSDGDT:   0,
            FSCLTX:   0,
            FSCATP:   0,
            FSCAUT:   0,
            FSCRSN:   0,
            FSSRDT:   0,
            FSCTLN:   0,
            FSDWNG:   0,
            FSDEVT:   0,
            FSCOP:    0,
            FSCPYS:   0,
            ENCRYP:   0,
            FBKGC:    0,
            ONAME:    0,
            OPHONE:   0,
            FL:       0,
            HL:       0,
            NUMI:     0,
            LISH_NNN: 0,
            LI_NNN:   0,
            ..self
        }
    }

    fn get_header_length_and_num_images(self, mut file: &File) -> NITF {
        let header_length_location =
            self.FHDR +
            self.FVER +
            self.CLEVEL +
            self.STYPE +
            self.OSTAID +
            self.FDT +
            self.FTITLE +
            self.FSCLAS +
            self.FSCLSY +
            self.FSCODE +
            self.FSCTLH +
            self.FSREL +
            self.FSDCTP +
            self.FSDCDT +
            self.FSDCXM +
            self.FSDG +
            self.FSDGDT +
            self.FSCLTX +
            self.FSCATP +
            self.FSCAUT +
            self.FSCRSN +
            self.FSSRDT +
            self.FSCTLN +
            self.FSDWNG +
            self.FSDEVT +
            self.FSCOP +
            self.FSCPYS +
            self.ENCRYP +
            self.FBKGC +
            self.ONAME +
            self.OPHONE +
            self.FL;
        
        let _ = file.seek(SeekFrom::Start(header_length_location.try_into().unwrap()));

        let header_length = read_int_from_file(&file, self.HL);
        let num_images = read_int_from_file(&file, self.NUMI);
        
        NITF {
            header_length: header_length,
            num_images: num_images,
            ..self
        }
    }

    fn save_images(self, mut file: &File) -> NITF {
        let mut image_header_lengths = std::vec::Vec::new(); 
        let mut image_data_lengths = std::vec::Vec::new(); 

        for _ in 0..self.num_images {
            let lish = read_int_from_file(&file, self.LISH_NNN);
            let li = read_int_from_file(&file, self.LI_NNN);
            
            image_header_lengths.push(lish);
            image_data_lengths.push(li);
        }

        let _ = file.seek(SeekFrom::Start(self.header_length.try_into().unwrap()));
        for image_number in 0..self.num_images {
            let mut image_header = vec![0u8; image_header_lengths[image_number]]; 
            let _ = file.read_exact(&mut image_header);

            let mut image_data = vec![0u8; image_data_lengths[image_number]];
            let _ = file.read_exact(&mut image_data);

            let path = format!("./testdata/image_{}.jp2", image_number + 1);
            let mut out_file = File::create(path).expect("Failed to create file: {path}");
            let _ = out_file.write_all(&image_data);
        }

        NITF {
            image_header_lengths: image_header_lengths,
            image_data_lengths: image_data_lengths,
            ..self
        }
    }
}


// variable input and output dirs
fn main() -> io::Result<()> {
    let nitf_file_path = "./testdata/input.nitf";
    let mut nitf_file = File::open(nitf_file_path).expect("Failed to open file: {nitf_file_path}.");

    let nitf = NITF { ..Default::default() };
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


fn read_string_from_file(mut file: &File, length: usize) -> String {
    let mut file_slice_bytes = vec![0u8; length];
    let _ = file.read_exact(&mut file_slice_bytes);
    let file_slice_str = String::from_utf8_lossy(&file_slice_bytes).to_string();
    file_slice_str
}


fn read_int_from_file(mut file: &File, length: usize) -> usize {
    let mut file_slice_bytes = vec![0u8; length];
    let _ = file.read_exact(&mut file_slice_bytes);
    let file_slice_str = String::from_utf8_lossy(&file_slice_bytes);
    let file_slice_int = file_slice_str
        .parse()
        .expect("File Slice String cannot be coerced to a number.");
        file_slice_int
}
