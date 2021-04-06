use crate::{decoder::Dtvcc, Result};

#[derive(Debug)]
enum C0Commands {
    NUL,
    ETX,
    BS,
    FF,
    CR,
    HCR,
    EXT1,
    P16,
}

impl C0Commands {
    pub fn new(c0: u8) -> Result<Self> {
        match c0 {
            0x00 => Ok(C0Commands::NUL),
            0x03 => Ok(C0Commands::ETX),
            0x08 => Ok(C0Commands::BS),
            0x0c => Ok(C0Commands::FF),
            0x0d => Ok(C0Commands::CR),
            0x0e => Ok(C0Commands::HCR),
            0x10 => Ok(C0Commands::EXT1),
            0x18 => Ok(C0Commands::P16),
            _ => Err("Invalid command")?,
        }
    }
}

#[derive(Debug)]
enum C1Commands {
    CW0,
    CW1,
    CW2,
    CW3,
    CW4,
    CW5,
    CW6,
    CW7,
    CLW,
    DSW,
    HDW,
    TGW,
    DLW,
    DLY,
    DLC,
    RST,
    SPA,
    SPC,
    SPL,
    RSV93,
    RSV94,
    RSV95,
    RSV96,
    SWA,
    DF0,
    DF1,
    DF2,
    DF3,
    DF4,
    DF5,
    DF6,
    DF7,
}

impl C1Commands {
    pub fn new(c1: u8) -> Result<Self> {
        match c1 {
            0x80 => Ok(C1Commands::CW0),
            0x81 => Ok(C1Commands::CW1),
            0x82 => Ok(C1Commands::CW2),
            0x83 => Ok(C1Commands::CW3),
            0x84 => Ok(C1Commands::CW4),
            0x85 => Ok(C1Commands::CW5),
            0x86 => Ok(C1Commands::CW6),
            0x87 => Ok(C1Commands::CW7),
            0x88 => Ok(C1Commands::CLW),
            0x89 => Ok(C1Commands::DSW),
            0x8A => Ok(C1Commands::HDW),
            0x8B => Ok(C1Commands::TGW),
            0x8C => Ok(C1Commands::DLW),
            0x8D => Ok(C1Commands::DLY),
            0x8E => Ok(C1Commands::DLC),
            0x8F => Ok(C1Commands::RST),
            0x90 => Ok(C1Commands::SPA),
            0x91 => Ok(C1Commands::SPC),
            0x92 => Ok(C1Commands::SPL),
            0x93 => Ok(C1Commands::RSV93),
            0x94 => Ok(C1Commands::RSV94),
            0x95 => Ok(C1Commands::RSV95),
            0x96 => Ok(C1Commands::RSV96),
            0x97 => Ok(C1Commands::SWA),
            0x98 => Ok(C1Commands::DF0),
            0x99 => Ok(C1Commands::DF1),
            0x9A => Ok(C1Commands::DF2),
            0x9B => Ok(C1Commands::DF3),
            0x9C => Ok(C1Commands::DF4),
            0x9D => Ok(C1Commands::DF5),
            0x9E => Ok(C1Commands::DF6),
            0x9F => Ok(C1Commands::DF7),
            _ => Err("Invalid command")?,
        }
    }
}

pub fn handle_c0(dtvcc: &mut Dtvcc, pos: usize, _block_length: u8) -> u8 {
    let mut len = 0;
    let c0 = C0Commands::new(dtvcc.current_packet[pos]);

    match c0 {
        Err(err) => eprintln!("{}", err),
        Ok(command) => {
            println!("[CEA-708] {:?}", command);
            match command {
                C0Commands::NUL => len = 1,
                C0Commands::ETX => len = 1,
                C0Commands::BS => len = 1,
                C0Commands::FF => len = 1,
                C0Commands::CR => len = 1,
                C0Commands::HCR => len = 1,
                C0Commands::EXT1 => len = 2,
                C0Commands::P16 => len = 3,
            }
        }
    }
    return len;
}

pub fn handle_c1(dtvcc: &mut Dtvcc, pos: usize, _block_length: u8) -> u8 {
    let mut len = 0;
    let c1 = C1Commands::new(dtvcc.current_packet[pos]);

    match c1 {
        Err(err) => eprintln!("{}", err),
        Ok(command) => {
            println!("[CEA-708] {:?}", command);
            match command {
                C1Commands::CW0 => len = 1,
                C1Commands::CW1 => len = 1,
                C1Commands::CW2 => len = 1,
                C1Commands::CW3 => len = 1,
                C1Commands::CW4 => len = 1,
                C1Commands::CW5 => len = 1,
                C1Commands::CW6 => len = 1,
                C1Commands::CW7 => len = 1,
                C1Commands::CLW => len = 2,
                C1Commands::DSW => len = 2,
                C1Commands::HDW => len = 2,
                C1Commands::TGW => len = 2,
                C1Commands::DLW => len = 2,
                C1Commands::DLY => len = 2,
                C1Commands::DLC => len = 1,
                C1Commands::RST => len = 1,
                C1Commands::SPA => len = 3,
                C1Commands::SPC => len = 4,
                C1Commands::SPL => len = 3,
                C1Commands::RSV93 => len = 1,
                C1Commands::RSV94 => len = 1,
                C1Commands::RSV95 => len = 1,
                C1Commands::RSV96 => len = 1,
                C1Commands::SWA => len = 5,
                C1Commands::DF0 => len = 7,
                C1Commands::DF1 => len = 7,
                C1Commands::DF2 => len = 7,
                C1Commands::DF3 => len = 7,
                C1Commands::DF4 => len = 7,
                C1Commands::DF5 => len = 7,
                C1Commands::DF6 => len = 7,
                C1Commands::DF7 => len = 7,
            }
        }
    }
    return len;
}

pub fn handle_g0(dtvcc: &mut Dtvcc, pos: usize, _block_length: u8) -> u8 {
    println!("[CEA-708] {}", dtvcc.current_packet[pos] as char);
    1
}

pub fn handle_g1(dtvcc: &mut Dtvcc, pos: usize, _block_length: u8) -> u8 {
    println!("[CEA-708] {}", dtvcc.current_packet[pos] as char);
    1
}

pub fn handle_c2(data: u8) -> u8 {
    println!("[CEA-708] C2 Command");
    if data <= 0x07 {
        // 00-07...
        return 1; // ... Single-byte control bytes (0 additional bytes)
    } else if data <= 0x0f {
        // 08-0F ..
        return 2; // ..two-byte control codes (1 additional byte)
    } else if data <= 0x17 {
        // 10-17 ...
        return 3; // ..three-byte control codes (2 additional bytes)
    }
    return 4; // 18-1F => four-byte control codes (3 additional bytes)
}

pub fn handle_c3(data: u8) -> u8 {
    println!("[CEA-708] C3 Command");
    if data < 0x80 || data > 0x9F {
        eprintln!("Out of range value");
    }
    if data <= 0x87 {
        // 80-87...
        return 5; // ... Five-byte control bytes (4 additional bytes)
    } else if data <= 0x8F {
        // 88-8F ...
        return 6; // ..Six-byte control codes (5 additional byte)
    }
    // If here, then 90-9F ...
    return 0;
}

pub fn handle_extended_char(dtvcc: &mut Dtvcc, pos: usize, _block_length: u8) -> u8 {
    let code = dtvcc.current_packet[pos];

    // Group C2
    if code <= 0x1F {
        handle_c2(code)
    }
    // Group G2 - Extended Miscellaneous Characters
    else if code >= 0x20 && code <= 0x7F {
        println!("[CEA-708] {}", code as char);
        1
    }
    // Group C3
    else if code >= 0x80 && code <= 0x9F {
        handle_c3(code)
    }
    // Group G3
    else {
        println!("[CEA-708] {}", code as char);
        1
    }
}
