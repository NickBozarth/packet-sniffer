use std::fmt::Debug;

use byte_slice::{Bytes, MacAddress, SliceToUnsigned};
use crate::packet::network::NetworkLayer;
use anyhow::Result;


#[derive(Debug, Default)]
pub enum DataLinkLayer<'a> {
    #[default]
    NULL,

    UndefinedData(&'a Bytes<'a>),
    ETHII(ETHII<'a>),
    PPP,
    HDLC,
}






#[derive(Default)]
pub struct Tag802_1Q(u16); // PCP 3 bits // DEI 1 bit // VID 12 bits

impl Debug for Tag802_1Q {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tag802_1Q")
            .field("PCP", &(self.0 >> 13))
            .field("DEI", &((self.0 >> 12) & 1))
            .field("VID", &(self.0 & 0x0fff))
            .finish()
    }
}


#[derive(Debug, Default)]
pub struct MacHeader {
    pub address_dst: MacAddress,
    pub address_src: MacAddress,
    pub tag_802_1q:  Option<Tag802_1Q>,
    pub ethertype:   u16,
}

impl MacHeader {
    pub fn from_bytes(bytes: &mut Bytes) -> Result<Self> {
        let address_dst = MacAddress::from(bytes[0..6].to_u64());
        let address_src = MacAddress::from(bytes[6..12].to_u64());
        let tag_802_1q = bytes[12..14]
            .eq(&[0x81, 0x00])
            .then(|| { Tag802_1Q(bytes[14..16].to_u16()) });
        let offset = if tag_802_1q.is_some() { 4 } else { 0 };
        let ethertype = bytes[12+offset..14+offset].to_u16();

        bytes.shift_first(14+offset)?;

        Ok(
            Self {
                address_dst,
                address_src,
                tag_802_1q,
                ethertype,
                ..Default::default()
            }
        )
    }
}







#[derive(Debug, Default)]
pub struct ETHII<'a> {
    pub mac_header: MacHeader,
    pub network_layer: NetworkLayer<'a>,
}

impl<'a> ETHII<'a> {
    // TODO this does not need to be pub if DataLinkLayer has a from_bytes function
    pub fn from_bytes(bytes: &'a mut Bytes) -> Result<Self> {
        let mac_header = MacHeader::from_bytes(bytes)?;
        let network_layer = NetworkLayer::from_data(mac_header.ethertype, bytes)?;

        Ok(
            Self {
                mac_header,
                network_layer
            }
        )
    }
}