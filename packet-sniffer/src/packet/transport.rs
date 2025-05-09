use byte_slice::{Bytes, SliceToUnsigned};
use anyhow::Result;


// https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml


#[derive(Debug, Default)]
pub enum TransportLayer<'a> {
    #[default]
    NULL,

    UndefinedData(&'a Bytes<'a>),
    TCP(TCP<'a>),
    UDP,
}

impl<'a> TransportLayer<'a> {
    pub fn from_data(layer_type: u8, bytes: &'a mut Bytes) -> Result<Self> {
        Ok(
            match layer_type {
                0x06 => Self::TCP(TCP::from_bytes(bytes)?),
                _ => Self::UndefinedData(bytes)
            }
        )
    }
}


#[derive(Debug, Default)]
pub struct TCP<'a> {
    pub port_src: u16,
    pub port_dst: u16,
    pub sequence_num: u32,
    pub acknowledgement_num: u32,
    pub data_offset: u8, // 4 bit // number of 32 bit words in the header //  DO / 4 = header_len (in bytes)
    pub reserved: u8, // 3 bit // ALWAYS 0
    pub control_bits: u8, // 6 bits
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
    // options


    pub data: &'a Bytes<'a>
}

impl<'a> TCP<'a> {
    pub fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {

        let port_src = bytes[0..2].to_u16();
        let port_dst = bytes[2..4].to_u16();
        let sequence_num = bytes[4..8].to_u32();
        let acknowledgement_num = bytes[8..12].to_u32();
        let data_offset = bytes[12] >> 4;
        let reserved = 0;
        let control_bits = (bytes[12..14].to_u16() & 0x3f) as u8;
        let window = bytes[14..16].to_u16();
        let checksum = bytes[16..18].to_u16();
        let urgent_ptr = bytes[18..20].to_u16();


        Ok(
            Self {
                port_src,
                port_dst,
                sequence_num,
                acknowledgement_num,
                data_offset,
                reserved,
                control_bits,
                window,
                checksum,
                urgent_ptr,
                data: bytes,

                ..Default::default()
            }
        )
    }
}