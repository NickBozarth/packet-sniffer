use byte_slice::{Bytes, Ipv4addr, Ipv6addr, MacAddress, SliceToUnsigned};
use crate::packet::transport::*;
use anyhow::Result;

#[derive(Debug, Default)]
pub enum NetworkLayer<'a> {
    #[default]
    NULL,

    UndefinedData(&'a Bytes<'a>),
    Ipv4(Ipv4<'a>),
    Ipv6(Ipv6<'a>),
    ARP(ARP),
    RARP,
    NAT,
    RIP,
    OSPF,
    BGP,
    IPSec,
    MPLS
}

impl<'a> NetworkLayer<'a> {
    pub fn from_data(layer_type: u16, bytes: &'a mut Bytes) -> Result<Self> {
        Ok(
            match layer_type {
                0x0800 => NetworkLayer::Ipv4(Ipv4::from_bytes(bytes)?),
                0x0806 => NetworkLayer::ARP(ARP::from_bytes(bytes)?),
                0x86dd => NetworkLayer::Ipv6(Ipv6::from_bytes(bytes)?),
                _ => NetworkLayer::UndefinedData(bytes)
            }
        )
    }
}






#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct Ipv4<'a> {
    pub version: u8,
    pub IHL: u8, // length of the header in 32 bit words
    pub DSPC: u8, // type of service
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8, // 0xx must be 0 // x1x Dont fragment // xx1 May fragment
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub address_src: Ipv4addr,
    pub address_dst: Ipv4addr,
    // pub options: Option<Vec<>>

    pub transport_layer: TransportLayer<'a>

}

impl<'a> Ipv4<'a> {
    #[allow(non_snake_case)]
    fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {
        let version = bytes[0] >> 4; // WHAT
        let IHL = bytes[0] & 0x0f;
        let DSPC = bytes[1];
        let total_length = bytes[2..4].to_u16();
        let identification = bytes[4..6].to_u16();
        let flags = bytes[6] >> 1;
        let fragment_offset = bytes[6..8].to_u16() & 0x1ff;
        let ttl = bytes[8];
        let protocol = bytes[9];
        let header_checksum = bytes[10..12].to_u16();
        let address_src = Ipv4addr(bytes[12..16].to_u32());
        let address_dst = Ipv4addr(bytes[16..20].to_u32());

        // TODO
        println!("TODO IMPLEMENT OPTIONS");

        bytes.shift_first(20)?;
        let transport_layer = TransportLayer::from_data(protocol, bytes)?;

        Ok(
            Self {
                version,
                IHL,
                DSPC,
                total_length,
                identification,
                flags,
                fragment_offset,
                ttl,
                protocol,
                header_checksum,
                address_src,
                address_dst,
                transport_layer,
            }
        )
    }
}





#[derive(Debug, Default)]
pub struct Ipv6<'a> {
    pub version: u8, // 4 bits
    pub traffic_class: u8,
    pub flow_label: u32, // 20 bits
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub address_src: Ipv6addr,
    pub address_dst: Ipv6addr,

    pub transport_layer: TransportLayer<'a>
}

impl<'a> Ipv6<'a> {
    fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {

        let version = bytes[0] >> 4;
        let traffic_class = (bytes[0..2].to_u16() >> 4) as u8;
        let flow_label = bytes[1..4].to_u32() & 0x0fffff;
        let payload_length = bytes[4..6].to_u16();
        let next_header = bytes[6];
        let hop_limit = bytes[7];
        let address_src = Ipv6addr(bytes[8..24].to_u128());
        let address_dst = Ipv6addr(bytes[24..40].to_u128());

        bytes.shift_first(40)?;




        let transport_layer = TransportLayer::from_data(next_header, bytes)?;

        Ok(
            Self {
                version,
                traffic_class,
                flow_label,
                payload_length,
                next_header,
                hop_limit,
                address_src,
                address_dst,
                transport_layer,

                ..Default::default()
            }
        )
    }
}






#[derive(Debug, Default)]
pub struct ARP {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hardware_len: u8,
    pub protocol_len: u8,
    pub operation: u16,
    pub sender_hardware_address: MacAddress, // 48 bits
    pub sender_protocol_address: Ipv4addr,
    pub target_hardware_address: MacAddress, // 48 bits
    pub target_protocol_address: Ipv4addr,
}

impl ARP {
    fn from_bytes(bytes: &Bytes) -> Result<Self> {

        let hardware_type = bytes[0..2].to_u16();
        let protocol_type = bytes[2..4].to_u16();
        let hardware_len = bytes[4];
        let protocol_len = bytes[5];
        let operation = bytes[6..8].to_u16();
        let sender_hardware_address = MacAddress::from(bytes[8..14].to_u64());
        let sender_protocol_address = Ipv4addr(bytes[14..18].to_u32());
        let target_hardware_address = MacAddress::from(bytes[18..24].to_u64());
        let target_protocol_address = Ipv4addr(bytes[24..28].to_u32());

        Ok(
            Self {
                hardware_type,
                protocol_type,
                hardware_len,
                protocol_len,
                operation,
                sender_hardware_address,
                sender_protocol_address,
                target_hardware_address,
                target_protocol_address
            }
        )
    }
}