use byte_slice::{Bytes, Ipv4addr, SliceToUnsigned};
use crate::packet::transport::*;
use anyhow::Result;

#[derive(Debug, Default)]
pub enum NetworkLayer<'a> {
    #[default]
    NULL,

    UndefinedData(&'a Bytes<'a>),
    Ipv4(Ipv4<'a>),
    Ipv6,
    ICMP,
    ARP,
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