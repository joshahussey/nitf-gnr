use std::time::Instant;

use rand::Rng;
use clap::{Arg, ArgAction, Command};
use chrono::{Utc, Datelike, Timelike};
use nitf_gnr::modify::parser::nitf21 as nitf;
use rayon::prelude::*;

fn main() {
    let matches = Command::new("nitf-gnr")
        .version("1.0")
        .author("josh <josh@nfld.com>")
        .about("A simple NITF generator to change NITF headers and output test files for large ingestion")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file path")
                .required(true)
                .value_parser(clap::value_parser!(std::string::String)),
        )
        .arg(
            Arg::new("output-prefix")
                .short('o')
                .long("output-prefix")
                .value_name("PREFIX")
                .help("Sets the output prefix")
                .required(true)
                .value_parser(clap::value_parser!(std::string::String)),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("NUMBER")
                .help("Sets the count")
                .required(true)
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("persistant")
                .short('p')
                .long("persistant")
                .value_name("DELAY")
                .help("Indicates persistant generation of NITF's. Sets the delay in minutes between generations")
                .required(false)
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("sequential")
                .short('s')
                .long("sequential")
                .help("Tells the generator not to parallelize the generation of NITF's")
                .action(ArgAction::SetTrue)
                .required(false)
        )
        .get_matches();

    let input_path = matches.get_one::<std::string::String>("input").unwrap().to_string();
    let output_prefix = matches.get_one::<std::string::String>("output-prefix").unwrap().to_string();
    let count: u32 = *matches.get_one("count").unwrap();
    let persistance: Option<&u32> = matches.get_one("persistant");
    let is_sequential = matches.get_flag("sequential");

    let descriptor = (persistance, is_sequential);
    match descriptor{
        (Some(p), false) => {
            loop {
                let start = Instant::now();
                println!("Generating {} NITF's", count);
                generate_nitfs(&input_path, &output_prefix, count);
                println!("Generated {} NITF's in {:.2?}", count, start.elapsed());
                let secs = (p*60) as u64;
                std::thread::sleep(std::time::Duration::from_secs(secs));
            }
        },
        (Some(p), true) => {
            loop {
                println!("Generating {} NITF's", count);
                let start = Instant::now();
                generate_nitfs_seq(&input_path, &output_prefix, count);
                println!("Generated {} NITF's in {:.2?}", count, start.elapsed());
                let secs = (p*60) as u64;
                std::thread::sleep(std::time::Duration::from_secs(secs));
            }
        },
        (None, false) => {
            println!("Generating {} NITF's", count);
            let start = Instant::now();
            generate_nitfs(&input_path, &output_prefix, count);
            println!("Generated {} NITF's in {:.2?}", count, start.elapsed());
        },
        (None, true) => {
            println!("Generating {} NITF's", count);
            let start = Instant::now();
            generate_nitfs_seq(&input_path, &output_prefix, count);
            println!("Generated {} NITF's in {:.2?}", count, start.elapsed());
        }
    }

}

fn generate_nitfs_seq(path: &str, o_prefix: &str, count: u32) {
    for _i in 0..count {
        _ = alter_nitf(path, o_prefix);
    };
}

fn generate_nitfs(path: &str, o_prefix: &str, count: u32) {
    (0..count).into_par_iter().for_each(|_i| {
        _ = alter_nitf(path, o_prefix);
    });
}

fn alter_nitf(path: &str, o_prefix: &str) -> std::string::String {
    let mut buf = std::fs::read(path).unwrap();
    let filename = change_filename(&mut buf);
    change_originator(&mut buf);
    change_fdt(&mut buf);
    change_ostaid(&mut buf);
    let path = o_prefix.to_string() + &filename;
    std::fs::write(path.as_str(), buf).expect("Unable to write file");
    filename
}

fn change_filename(buf: &mut [u8]) -> std::string::String {
    use nitf::NitfHeader21 as N;
    let filename = generate_filename();
    let ft_offset = N::get_offset(N::FTITLE, None);
    buf[ft_offset..ft_offset + 80].copy_from_slice(filename.as_bytes());
    filename
}

fn change_originator(buf: &mut [u8]) {
    use nitf::NitfHeader21 as N;
    //let user = get_current_username().unwrap();
    let user = whoami::username();
    let mut o_buf: [u8; 24] = [32; 24];
    let mut originator = user.as_str();
    let o_len = originator.len();
    if o_len > 24 {
        originator = &originator[..originator.char_indices().nth(24).map_or(originator.len(), |(idx, _)| idx)];
    } 
    o_buf[..originator.len()].copy_from_slice(originator.as_bytes());
    let originator_offset = N::get_offset(N::ONAME, None);
    buf[originator_offset..originator_offset + 24].copy_from_slice(&o_buf);
}

fn change_fdt(buf: &mut [u8]) {
    use nitf::NitfHeader21 as N;
    let now = Utc::now(); 
    let fdt = format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    let fdt_offset = N::get_offset(N::FDT, None);
    buf[fdt_offset..fdt_offset + 14].copy_from_slice(fdt.as_bytes());
}

fn change_ostaid(buf: &mut [u8]) {
    use nitf::NitfHeader21 as N;
    let ostaid = "COMPUSULT ";
    let ostaid_offset = N::get_offset(N::OSTAID, None);
    buf[ostaid_offset..ostaid_offset + 10].copy_from_slice(ostaid.as_bytes());
}

fn generate_filename() -> std::string::String {
    let random_u8: Vec<u8> = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(61)
        .collect();
    let mut ft_buf: [u8; 80] = [0; 80];
    let bytes_cslt_test = [
        99, 115, 108, 116, 45, 116, 101, 115, 116, 45, 110, 105, 116, 102, 45
    ];
    let bytes_extension = [46, 110, 116, 102];
    ft_buf[..15].copy_from_slice(&bytes_cslt_test);
    ft_buf[76..].copy_from_slice(&bytes_extension);
    ft_buf[15..76].copy_from_slice(random_u8.as_slice());
    let name_string = match std::str::from_utf8(&ft_buf) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error: {}\n Skipping this file.", e);
        }
    };
    name_string.to_string()
}
