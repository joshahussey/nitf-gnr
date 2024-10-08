use std::io::Read;

//helper functions
pub fn calculate_file_crc32(file_path: &str) -> std::io::Result<u32> {
    let mut file = std::fs::File::open(file_path)?;
    let mut hasher = crc32fast::Hasher::new();
    let mut buffer = [0; 4096];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize())
}

pub fn calculate_bytes_crc32(data: &Vec<u8>) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    hasher.finalize()
}
