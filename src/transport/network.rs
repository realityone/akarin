use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use common::error::*;

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct IPv4Header {
    pub version_ihl: u8, // IP version (= 4) + Internet header length
    pub type_of_service: u8, // Type of service
    pub total_length: u16, // Total length in octets
    pub identification: u16, // Identification
    pub flags_fragment_offset: u16, // 3-bits Flags + Fragment Offset
    pub time_to_live: u8, // Time To Live
    pub protocol: u8, // Protocol
    pub header_checksum: u16, // Checksum
    pub source_address: u32, // Source Address
    pub destination_address: u32, // Destination Address
}

impl From<[u8; 20]> for IPv4Header {
    fn from(bytes: [u8; 20]) -> Self {
        let mut bytes = Cursor::new(bytes);
        let mut hdr = IPv4Header::default();
        hdr.version_ihl = bytes.read_u8().unwrap();
        hdr.type_of_service = bytes.read_u8().unwrap();
        hdr.total_length = bytes.read_u16::<BigEndian>().unwrap();
        hdr.identification = bytes.read_u16::<BigEndian>().unwrap();
        hdr.flags_fragment_offset = bytes.read_u16::<BigEndian>().unwrap();
        hdr.time_to_live = bytes.read_u8().unwrap();
        hdr.protocol = bytes.read_u8().unwrap();
        hdr.header_checksum = bytes.read_u16::<BigEndian>().unwrap();
        hdr.source_address = bytes.read_u32::<BigEndian>().unwrap();
        hdr.destination_address = bytes.read_u32::<BigEndian>().unwrap();
        hdr
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem;
    use std::net::Ipv4Addr;
    use std::str::FromStr;

    #[test]
    fn test_ipv4_header() {
        assert_eq!(mem::size_of::<IPv4Header>(), 20);

        let data = [
            69,
            0,
            0,
            84,
            223,
            18,
            64,
            0,
            64,
            1,
            143,
            234,
            10,
            0,
            0,
            2,
            192,
            168,
            1,
            2,
        ];
        let h = IPv4Header::from(data);
        assert_eq!(h.protocol, 0x01);
        assert_eq!(Ipv4Addr::from(h.source_address), Ipv4Addr::from_str("10.0.0.2").unwrap());
    }
}