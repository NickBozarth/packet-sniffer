#![allow(dead_code)]


use std::{fmt::Debug, net::Ipv4Addr};
use anyhow::{Result, anyhow};

type MacAddress = [u8;6];

#[derive(Debug, Default)]
pub enum ProtocolHeader<'a> {
    #[default]
    UNDEFINED,

    IPv4(Ipv4Header<'a>),
    IPv6(i32),
    ARP(i32),
    VLAN(i32),
    LLDP(i32),
}

impl<'a> ProtocolHeader<'a> {
    fn from_bytes(protocol_type: u16, bytes: &[u8]) -> Result<(ProtocolHeader, &[u8])> {
        Ok(
            match protocol_type {
                0x0800 => {
                    let (header, unused_data) = Ipv4Header::from_bytes(bytes)?;
                    (ProtocolHeader::IPv4(header), unused_data)
                }
                0x86DD => (ProtocolHeader::IPv6(0), bytes),
                0x0806 => (ProtocolHeader::ARP(0), bytes),
                0x8100 => (ProtocolHeader::VLAN(0), bytes),
                0x88CC => (ProtocolHeader::LLDP(0), bytes),
                _      => (ProtocolHeader::UNDEFINED, bytes),
            }
        )
    }
}


#[derive(Debug, Default)]
pub enum Protocol {
    #[default]
    UNDEFINED,

    ICMP,
    IGMP,
    TCP,
    UDP,
    ENCAP,
    OSPF,
    SCTP,
}

impl Protocol {
    pub fn from_value(value: u8) -> Self {
        match value {
            0x01 => Protocol::ICMP,
            0x02 => Protocol::IGMP,
            0x06 => Protocol::TCP,
            0x11 => Protocol::UDP,
            0x29 => Protocol::ENCAP,
            0x59 => Protocol::OSPF,
            0x84 => Protocol::SCTP,
            _    => Protocol::UNDEFINED
        }
    }
}



trait ByteFormatters {
    fn as_decimal<T>(&self) -> Result<T>
    where T: TryFrom<u128>;

    fn as_bits_string(&self) -> String;
}

impl ByteFormatters for [u8] {
    fn as_decimal<T>(&self) -> Result<T>
    where 
        T: TryFrom<u128> 
    {
        let mut ret: u128 = 0;
        for byte in self {
            ret *= 256;
            ret += *byte as u128;
        }

        T::try_from(ret)
            .map_err(|_| anyhow!("Could not convert [u8] to T"))
    }

    fn as_bits_string(&self) -> String {
        let mut ret_string = "[".to_owned();
        for byte in self {
            ret_string.push_str(&format!("{:08b}, ", byte));
        }
        ret_string.pop();
        ret_string.pop();
        ret_string.push(']');
        ret_string
    }
}




#[derive(Debug)]
pub struct Ipv4Header<'a> {
    pub version: u8, // should always be 4 because it is an ipv4 packet
    pub ihl: u8, // internet header length

    pub dscp: u8, // differentiated services code point
    pub ecn: u8, //explicit congestion notification

    pub total_length: u16, // length of the packet
    pub identification: u16, // primarily used for uniquely identifying the group of fragments of a single IP datagram???

    pub flags: u8, // 3 bits // first always 0 // second is flagged for no fragment // for fragmented packets, all but the last should have this flagged
    pub fragment_offset: u16, // offset of the packet as compared to all other packets // first packet will always be 0 // middle packets all multiples of 8 // last may be any number

    pub ttl: u8, // time to live in seconds although, routers typically just decrement the value by 1 instead so essentially a hop count
    pub protocol: Protocol,

    pub checksum: u16, // used for error handling // computed by each router when routing packet

    pub address_src: Ipv4Addr,
    pub address_dst: Ipv4Addr,

    pub options: Option<&'a [u8]>
}

impl<'a> Ipv4Header<'a> {
    // create a header from bytes and return unused data
    pub fn from_bytes(bytes: &'a [u8]) -> Result<(Self, &'a [u8])> {
        println!("Bytes: {bytes:X?}");
        println!("bits: {:?}", bytes.as_bits_string());

        let mut packet = Ipv4Header::default();

        packet.version = bytes[0] >> 4;
        packet.ihl     = bytes[0] & 0x0f;

        packet.dscp = bytes[1] >> 2;
        packet.ecn  = bytes[1] & 0x03;

        packet.total_length   = bytes[2..4].as_decimal()?;
        packet.identification = bytes[4..6].as_decimal()?;

        packet.flags           = bytes[6] >> 5;
        packet.fragment_offset = bytes[6..8].as_decimal::<u16>()? & 0x1fff;

        packet.ttl      = bytes[8];
        packet.protocol = Protocol::from_value(bytes[9]);

        packet.checksum = bytes[10..12].as_decimal()?;

        packet.address_src = Ipv4Addr::from_bits(bytes[12..16].as_decimal()?);
        packet.address_dst = Ipv4Addr::from_bits(bytes[16..20].as_decimal()?);

        if packet.ihl > 5 {
            packet.options = Some(&bytes[20..(packet.ihl as usize * 4_usize)]);
        }

        let unused_bytes = &bytes[(packet.ihl as usize * 4_usize)..];
        Ok((packet, unused_bytes))
    }
}

impl<'a> Default for Ipv4Header<'a> {
    fn default() -> Self {
        Self { 
            version: 0, 
            ihl: 0, 
            dscp: 0, 
            ecn: 0, 
            total_length: 0, 
            identification: 0, 
            flags: 0, 
            fragment_offset: 0, 
            ttl: 0, 
            protocol: Protocol::default(), 
            checksum: 0, 
            address_src: Ipv4Addr::from_bits(0x00000000), 
            address_dst: Ipv4Addr::from_bits(0x00000000),
            options: None
        }
    }
}





#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct Packet<'a> {
    pub data: &'a [u8],
    pub ethII_dst: MacAddress,
    pub ethII_src: MacAddress,
    pub header: ProtocolHeader<'a>,
    pub body: &'a [u8]
}

impl<'a> Packet<'a> {
    pub fn from_str(value: &'a str) -> Result<Self> {
        let mut packet = Self { 
            data: value.as_bytes(), 
            ..Default::default() 
        };

        packet.initialize()?;
        Ok(packet)
    }

    pub fn from_bytes(slice: &'a[u8]) -> Result<Self> {
        let mut packet = Self { 
            data: slice, 
            ..Default::default() 
        };

        packet.initialize()?;
        Ok(packet)
    }

    fn initialize(&mut self) -> Result<&mut Self> {
        self.ethII_dst = self.data[0..6]
            .try_into()
            .map_err(|_| anyhow!("Failed to retrieve MAC address from packet"))?;

        self.ethII_src = self.data[6..12]
            .try_into()
            .map_err(|_| anyhow!("Failed to retrieve MAC address from packet"))?;

        let header_protocol_slice = &self.data[12..14];
        let header_protocol_decimal = header_protocol_slice.as_decimal::<u16>()?;
        let (header, body) = ProtocolHeader::from_bytes(header_protocol_decimal, &self.data[14..])?;
        self.header = header;
        self.body = body;

        Ok(self)
    }
}