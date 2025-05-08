use packet_sniffer::packet::data_link::{self, ETHII};
use byte_slice::*;
use anyhow::Result;

fn main() -> Result<()> {
    let ipv4_raw_packet_data: [u8; 55] = [0x8, 0x26, 0x97, 0x6c, 0x21, 0x40, 0x20, 0x4e, 0xf6, 0x34, 0xc6, 0xe3, 0x8, 0x0, 0x45, 0x0, 0x0, 0x29, 0xa3, 0xb2, 0x40, 0x0, 0x80, 0x6, 0xd4, 0xa8, 0xc0, 0xa8, 0x1, 0x6a, 0xd, 0x59, 0xb3, 0x8, 0xfe, 0xa8, 0x1, 0xbb, 0xcb, 0x83, 0xaf, 0xdd, 0xe7, 0x7a, 0xff, 0x0, 0x50, 0x10, 0x0, 0xfb, 0xca, 0x23, 0x0, 0x0, 0x0];

    let mut ipv4_bytes = Bytes::from_slice(&ipv4_raw_packet_data);
    let ethernet_layer = ETHII::from_bytes(&mut ipv4_bytes)?;

    println!("ethernet_layer = {ethernet_layer:x?}");

    Ok(())
}


#[test]
fn test() -> Result<()> {
    // rand_cap_2.pcapng 20
    let tcp_packet_with_substantial_data = "204ef634c6e30826976c21400800450000d3dafe40006d063bea68d0cb59c0a8016a01bbc92111191d5bc88f07b450182000147c000017030300a6000000000000000d22c4cb9cf3dfdfb75480ff95559f43527c4b930ddb6a1699c0125885ff792de0f7ed4c9c672593b36089812f7f9313a64f97515d27edb4f7be2124bf689d8eadd67d268f4001a3fbb6c637c7b208654177c188cc0550c57a5729405e40ee28c46a1515c16da38dfda9e080dcdc1a1b496d253bea7d62d69a3fea719c0547bc1a64d8000a746a95d33fe19c096b08ffd7e1d88b6a96fbbffcdeea14ba9f73"
        .hex_stream_to_vec();

    let mut bytes = Bytes::from_slice(&tcp_packet_with_substantial_data);
    let ethii = ETHII::from_bytes(&mut bytes)?;

    println!("{:x?}", ethii);


    Ok(())
}