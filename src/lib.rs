use std::{env, fs::File, io::Read, u16};
use std::{error::Error, io::BufReader};

mod commands;
mod decoder;

use decoder::Decoder;

// create an alias for the result type
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn run() -> Result<()> {
    let file_path = env::args()
        .nth(1)
        .ok_or("path to input file is missing\nUsage: cea_708_decoder <path to input file>")?;

    parse_file(&file_path)?;
    Ok(())
}

/// Parse RCWT file
fn parse_file(file_path: &str) -> Result<()> {
    let mut buf_reader = BufReader::new(File::open(file_path)?);
    let mut decoder = Decoder::new();
    parse_header(&mut buf_reader)?;

    let mut buffer = [0u8; 10];
    while let Ok(_) = buf_reader.read_exact(&mut buffer) {
        let fts = u64::from_ne_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7],
        ]);
        let data_len: usize = u16::from_ne_bytes([buffer[8], buffer[9]]) as usize * 3;
        let mut data_buf = vec![0u8; data_len];
        buf_reader.read_exact(&mut data_buf)?;
        parse_data(&mut decoder, &data_buf)?;
    }

    Ok(())
}

/// Parse RCWT header
fn parse_header(reader: &mut BufReader<File>) -> Result<()> {
    let mut buffer = [0u8; 11];
    reader.read_exact(&mut buffer)?;

    // Make sure magic number and reserved bytes are correct
    assert_eq!(&[0xcc, 0xcc, 0xed, 0xcc], &buffer[0..=3]); //magic number
    assert_eq!(&[0, 0, 0], &buffer[8..=10]); //reserved

    let program_version = u16::from_be_bytes([buffer[4], buffer[5]]);
    let file_format = u16::from_be_bytes([buffer[6], buffer[7]]);

    println!("Program version:- {}", program_version);
    println!("File format:- {}", file_format);
    Ok(())
}

/// Parse caption data
fn parse_data(decoder: &mut Decoder, data: &Vec<u8>) -> Result<()> {
    let data: Vec<&[u8]> = data.chunks(3).collect();
    for caption_data in data {
        let cc_valid = (caption_data[0] & 4) >> 2;
        let cc_type = caption_data[0] & 3;

        if cc_valid == 1 {
            if cc_type == 2 || cc_type == 3 {
                decoder::ccx_dtvcc_process_data(decoder, cc_valid, cc_type, &caption_data[1..=2])?;
            }
        }
    }
    Ok(())
}
