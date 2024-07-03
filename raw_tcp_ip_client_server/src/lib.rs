pub fn htons(u: u16) -> u16 {
    u16::to_be(u)
}

pub fn htonl(u: u32) -> u32 {
    u32::to_be(u)
}
