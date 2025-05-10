use byte_slice::{Bytes, SliceToUnsigned};
use anyhow::Result;


// https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml


#[derive(Debug, Default)]
pub enum TransportLayer<'a> {
    #[default]
    NULL,

    UndefinedData(&'a Bytes<'a>),
    ICMP(ICMP<'a>),
    TCP(TCP<'a>),
    UDP(UDP<'a>),
}

impl<'a> TransportLayer<'a> {
    pub fn from_data(layer_type: u8, bytes: &'a mut Bytes) -> Result<Self> {
        Ok(
            match layer_type {
                0x01 => Self::ICMP(ICMP::from_bytes(bytes)?),
                0x06 => Self::TCP(TCP::from_bytes(bytes)?),
                0x11 => Self::UDP(UDP::from_bytes(bytes)?),
                _ => Self::UndefinedData(bytes)
            }
        )
    }
}




#[derive(Debug, Default)]
pub enum MessageType {
    #[default]
    UndefinedType         = 0xff,

    EchoReply             = 0,
    DesinationUnreachable = 3,
    SourceQuench          = 4,
    Redirect              = 5,
    EchoRequest           = 8,
    TimeExceeded          = 11,
    ParameterProblem      = 12,
    Timestamp             = 13,
    TimestampReply        = 14,
    InformationRequest    = 15,
    InformationReply      = 16,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 =>  MessageType::EchoReply, 
            3 =>  MessageType::DesinationUnreachable, 
            4 =>  MessageType::SourceQuench, 
            5 =>  MessageType::Redirect, 
            8 =>  MessageType::EchoRequest, 
            11 => MessageType::TimeExceeded, 
            12 => MessageType::ParameterProblem, 
            13 => MessageType::Timestamp, 
            14 => MessageType::TimestampReply, 
            15 => MessageType::InformationRequest, 
            16 => MessageType::InformationReply, 
            _ =>  MessageType::UndefinedType,
        }
    }
}



#[derive(Debug, Default)]
pub struct ICMP<'a> {
    pub message_type: MessageType,
    pub code: u8,
    pub checksum: u16,


    pub payload: &'a Bytes<'a>
}

impl<'a> ICMP<'a> {
    fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {
        let message_type = bytes[0].into();
        let code = 0;
        let checksum = 0;

        Ok(
            Self {
                message_type,
                code,
                checksum,

                payload: bytes
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
    pub data_offset: u8, // 4 bit // number of 32 bit words in the header //  data_offset / 4 = header_len (in bytes)
    pub reserved: u8, // 3 bit // ALWAYS 0
    pub control_bits: u8, // 6 bits
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
    // options


    pub payload: &'a Bytes<'a>
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
                payload: bytes,

                ..Default::default()
            }
        )
    }
}




#[derive(Debug, Default)]
pub struct UDP<'a> {
    pub port_src: u16,
    pub port_dst: u16,
    pub length: u16,
    pub checksum: u16,

    pub payload: &'a Bytes<'a>,
}

impl<'a> UDP<'a> {
    fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {
        let port_src = bytes[0..2].to_u16();
        let port_dst = bytes[2..4].to_u16();
        let length = bytes[4..6].to_u16();
        let checksum = bytes[6..8].to_u16();

        bytes.shift_first(8)?;

        Ok(
            Self{
                port_src,
                port_dst,
                length,
                checksum,

                payload: bytes
            }
        )
    }
}