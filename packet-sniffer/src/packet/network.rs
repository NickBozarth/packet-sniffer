use byte_slice::Bytes;



#[derive(Debug, Default)]
pub enum NetworkLayer {
    #[default]
    UNDEFINED,

    Ipv4(Ipv4),
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

impl NetworkLayer {
    pub fn from_data(layer_type: u16, bytes: &mut Bytes) -> Self {
        match layer_type {
            0x0800 => NetworkLayer::Ipv4(Ipv4::from_bytes(bytes)),
            _ => NetworkLayer::UNDEFINED
        }
    }
}



#[derive(Debug, Default)]
pub struct Ipv4 {

}

impl Ipv4 {
    fn from_bytes(bytes: &mut Bytes) -> Self {
        todo!()
    }
}