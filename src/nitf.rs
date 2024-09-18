use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

const FHDR: usize = 4;
const FVER: usize = 5;

#[derive(Default, Debug)]
pub struct Nitf {
    // header values
    fhdr:     usize,
    fver:     usize,
    clevel:   usize,
    stype:    usize,
    ostaid:   usize,
    fdt:      usize,
    ftitle:   usize,
    fsclas:   usize,
    fsclsy:   usize,
    fscode:   usize,
    fsctlh:   usize,
    fsrel:    usize,
    fsdctp:   usize,
    fsdcdt:   usize,
    fsdcxm:   usize,
    fsdg:     usize,
    fsdgdt:   usize,
    fscltx:   usize,
    fscatp:   usize,
    fscaut:   usize,
    fscrsn:   usize,
    fssrdt:   usize,
    fsctln:   usize,
    fsdwng:   usize,
    fsdevt:   usize,
    fscop:    usize,
    fscpys:   usize,
    encryp:   usize,
    fbkgc:    usize,
    oname:    usize,
    ophone:   usize,
    fl:       usize,
    hl:       usize,
    numi:     usize,
    lish_nnn: usize,
    li_nnn:   usize,
    // non-header values
    file_profile_name: String,
    file_version: String,
    header_length: usize,
    num_images: usize,
    image_header_lengths: Vec<usize>,
    image_data_lengths: Vec<usize>
}

impl Nitf {
    pub fn get_file_profile_name_and_version(self, file: &File) -> Nitf {
        let file_profile_name = read_string_from_file(&file, FHDR);
        let file_version = read_string_from_file(&file, FVER);

        Nitf {
            file_profile_name,
            file_version,
            ..self
        }
    }

    fn load_v02_10(self) -> Nitf {
        Nitf {
            fhdr:      4,
            fver:      5,
            clevel:    2,
            stype:     4,
            ostaid:   10,
            fdt:      14,
            ftitle:   80,
            fsclas:    1,
            fsclsy:    2,
            fscode:   11,
            fsctlh:    2,
            fsrel:    20,
            fsdctp:    2,
            fsdcdt:    8,
            fsdcxm:    4,
            fsdg:      1,
            fsdgdt:    8,
            fscltx:   43,
            fscatp:    1,
            fscaut:   40,
            fscrsn:    1,
            fssrdt:    8,
            fsctln:   15,
            fscop:     5,
            fscpys:    5,
            encryp:    1,
            fbkgc:     3,
            oname:    24,
            ophone:   18,
            fl:       12,
            hl:        6,
            numi:      3,
            lish_nnn:  6,
            li_nnn:   10,
            ..self
        }
    }

    fn load_v02_00(self) -> Nitf {
        Nitf {
            fhdr:      9,
            clevel:    2,
            stype:     4,
            ostaid:   10,
            fdt:      14,
            ftitle:   80,
            fsclas:    1,
            fscode:   40,
            fsctlh:   40,
            fsrel:    40,
            fscaut:   20,
            fsctln:   20,
            fsdwng:    6,
            fsdevt:   40,
            fscop:     5,
            fscpys:    5,
            encryp:    1,
            oname:    27,
            ophone:   18,
            fl:       12,
            hl:        6,
            numi:      3,
            lish_nnn:  6,
            li_nnn:   10,
            ..self
        }
    }

    // not implemented
    fn load_v01_10(self) -> Nitf {
        Nitf {
            fhdr:     0,
            fver:     0,
            clevel:   0,
            stype:    0,
            ostaid:   0,
            fdt:      0,
            ftitle:   0,
            fsclas:   0,
            fsclsy:   0,
            fscode:   0,
            fsctlh:   0,
            fsrel:    0,
            fsdctp:   0,
            fsdcdt:   0,
            fsdcxm:   0,
            fsdg:     0,
            fsdgdt:   0,
            fscltx:   0,
            fscatp:   0,
            fscaut:   0,
            fscrsn:   0,
            fssrdt:   0,
            fsctln:   0,
            fsdwng:   0,
            fsdevt:   0,
            fscop:    0,
            fscpys:   0,
            encryp:   0,
            fbkgc:    0,
            oname:    0,
            ophone:   0,
            fl:       0,
            hl:       0,
            numi:     0,
            lish_nnn: 0,
            li_nnn:   0,
            ..self
        }
    }

    fn get_header_length_and_num_images(self, mut file: &File) -> Nitf {
        let header_length_location =
            self.fhdr +
            self.fver +
            self.clevel +
            self.stype +
            self.ostaid +
            self.fdt +
            self.ftitle +
            self.fsclas +
            self.fsclsy +
            self.fscode +
            self.fsctlh +
            self.fsrel +
            self.fsdctp +
            self.fsdcdt +
            self.fsdcxm +
            self.fsdg +
            self.fsdgdt +
            self.fscltx +
            self.fscatp +
            self.fscaut +
            self.fscrsn +
            self.fssrdt +
            self.fsctln +
            self.fsdwng +
            self.fsdevt +
            self.fscop +
            self.fscpys +
            self.encryp +
            self.fbkgc +
            self.oname +
            self.ophone +
            self.fl;
        
        let _ = file.seek(SeekFrom::Start(header_length_location.try_into().unwrap()));

        let header_length = read_int_from_file(&file, self.hl);
        let num_images = read_int_from_file(&file, self.numi);
        
        Nitf {
            header_length,
            num_images,
            ..self
        }
    }

    fn save_images(self, mut file: &File) -> Nitf {
        let mut image_header_lengths = std::vec::Vec::new(); 
        let mut image_data_lengths = std::vec::Vec::new(); 

        file.seek(SeekFrom::Start(self.hl as u64)).expect("Failed to seek to start of file.");
        for _ in 0..self.num_images {
            let lish = read_int_from_file(&file, self.lish_nnn);
            let li = read_int_from_file(&file, self.li_nnn);
            
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

        Nitf {
            image_header_lengths,
            image_data_lengths,
            ..self
        }
    }
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

fn read_string_from_file(mut file: &File, length: usize) -> String {
    let mut file_slice_bytes = vec![0u8; length];
    let _ = file.read_exact(&mut file_slice_bytes);
    let file_slice_str = String::from_utf8_lossy(&file_slice_bytes).to_string();
    file_slice_str
}
