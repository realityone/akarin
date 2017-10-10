use std::mem;

#[repr(C, packed)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ipv4_header() {
        assert_eq!(mem::size_of::<IPv4Header>(), 20);
    }
}