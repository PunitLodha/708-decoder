use crate::{commands, Result};

pub struct Decoder {
    dtvcc: Dtvcc,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            dtvcc: Dtvcc::new(),
        }
    }
}

pub struct Dtvcc {
    pub current_packet: Vec<u8>,
    pub current_packet_length: u8,
}

impl Dtvcc {
    pub fn new() -> Self {
        Self {
            current_packet: vec![],
            current_packet_length: 0,
        }
    }
}

pub fn ccx_dtvcc_process_data(
    decoder: &mut Decoder,
    cc_valid: u8,
    cc_type: u8,
    data: &[u8],
) -> Result<()> {
    let dtvcc = &mut decoder.dtvcc;
    if cc_type == 2 {
        if cc_valid == 0 {
            ccx_dtvcc_process_current_packet(dtvcc)
        } else {
            if dtvcc.current_packet_length > 253 {
                eprintln!("Warning: Legal packet size exceeded (1), data not added.");
            } else {
                dtvcc.current_packet.extend_from_slice(data);
                dtvcc.current_packet_length += 2;
            }
        }
    } else if cc_type == 3 {
        ccx_dtvcc_process_current_packet(dtvcc);
        if cc_valid == 1 {
            if dtvcc.current_packet_length > (128 - 1) {
                eprintln!("Warning: Legal packet size exceeded (2), data not added.");
            } else {
                dtvcc.current_packet.extend_from_slice(data);
                dtvcc.current_packet_length += 2;
            }
        }
    }

    Ok(())
}

pub fn ccx_dtvcc_process_current_packet(dtvcc: &mut Dtvcc) {
    if dtvcc.current_packet.len() > 0 {
        let seq = (dtvcc.current_packet[0] & 0xC0) >> 6;
        let mut len = dtvcc.current_packet[0] & 0x3F;

        if dtvcc.current_packet_length == 0 {
            return;
        }
        if len == 0 {
            // This is well defined in EIA-708; no magic.
            len = 128;
        } else {
            len = len * 2;
        }

        if dtvcc.current_packet_length != len {
            // Most likely things are going to be bad for us
            len = dtvcc.current_packet_length; // At least don't read beyond the buffer
        }

        let mut pos = 1;
        while pos < len {
            let mut service_number = (dtvcc.current_packet[pos as usize] & 0xE0) >> 5; // 3 more significant bits
            let block_length = dtvcc.current_packet[pos as usize] & 0x1F; // 5 less significant bits

            if service_number == 7 {
                pos += 1;
                service_number = dtvcc.current_packet[pos as usize] & 0x3F;
            }

            pos += 1;

            if service_number == 0 && block_length != 0 {
                // Illegal, but specs say what to do...
                pos = len; // Move to end
                break;
            }

            if service_number > 0 {
                ccx_dtvcc_process_service_block(dtvcc, pos, block_length);
            }

            pos += block_length // Skip data
        }

        dtvcc.current_packet = vec![];
        dtvcc.current_packet_length = 0;
    }
}

pub fn ccx_dtvcc_process_service_block(dtvcc: &mut Dtvcc, pos: u8, block_length: u8) {
    let mut i = 0;
    while i < block_length {
        let curr: usize = (pos + i) as usize;

        // 16 for EXT1
        let consumed = if dtvcc.current_packet[curr] != 16 {
            if dtvcc.current_packet[curr] <= 0x1F {
                commands::handle_c0(dtvcc, curr, block_length - i)
            } else if dtvcc.current_packet[curr] >= 0x20 && dtvcc.current_packet[curr] <= 0x7F {
                commands::handle_g0(dtvcc, curr, block_length - i)
            } else if dtvcc.current_packet[curr] >= 0x80 && dtvcc.current_packet[curr] <= 0x9F {
                commands::handle_c1(dtvcc, curr, block_length - i)
            } else {
                commands::handle_g1(dtvcc, curr, block_length - i)
            }
        } else {
            let mut used = commands::handle_extended_char(dtvcc, curr, block_length - i);
            used += 1;
            used // Since we had CCX_DTVCC_C0_EXT1
        };
        i += consumed;
    }
}
