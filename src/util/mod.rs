use byteorder::{BigEndian, ByteOrder};
use error;
use nom::{be_u16, be_u32, be_u64};

named!(inner_take_u16 <&[u8], u16>, map!(take!(2), |i| be_u16(i).unwrap().1));
named!(inner_take_u32 <&[u8], u32>, map!(take!(4), |i| be_u32(i).unwrap().1));
named!(inner_take_u64 <&[u8], u64>, map!(take!(8), |i| be_u64(i).unwrap().1));

pub fn take_u16(i: &[u8]) -> error::ParseResult<u16> {
    error::to_result(inner_take_u16(i))
}
pub fn take_u32(i: &[u8]) -> error::ParseResult<u32> {
    error::to_result(inner_take_u32(i))
}
pub fn take_u64(i: &[u8]) -> error::ParseResult<u64> {
    error::to_result(inner_take_u64(i))
}

pub fn take_u128(i: &[u8]) -> error::ParseResult<u128> {
    if i.len() < 16 {
        Err(error::Error::InvalidLength)
    } else {
        let res = ((i[0] as u128) << 120)
            + ((i[1] as u128) << 112)
            + ((i[2] as u128) << 104)
            + ((i[3] as u128) << 96)
            + ((i[4] as u128) << 88)
            + ((i[5] as u128) << 80)
            + ((i[6] as u128) << 72)
            + ((i[7] as u128) << 64)
            + ((i[8] as u128) << 56)
            + ((i[9] as u128) << 48)
            + ((i[10] as u128) << 40)
            + ((i[11] as u128) << 32)
            + ((i[12] as u128) << 24)
            + ((i[13] as u128) << 16)
            + ((i[14] as u128) << 8)
            + (i[15] as u128);
        Ok((&i[16..], res))
    }
}

pub fn u16_to_bytes(src: u16, dst: &mut [u8; 2]) {
    BigEndian::write_u16_into(&[src], dst);
}

pub fn u32_to_bytes(src: u32, dst: &mut [u8; 4]) {
    BigEndian::write_u32_into(&[src], dst);
}

pub fn u64_to_bytes(src: u64, dst: &mut [u8; 8]) {
    BigEndian::write_u64_into(&[src], dst);
}

pub fn u128_to_bytes(src: u128, dst: &mut [u8; 16]) {
    for i in 0..16 {
        dst[15 - i] = (src >> (8 * i) & 0xff) as u8;
    }
}

#[cfg(test)]
mod test_util {
    use util;

    #[test]
    fn test_take() {
        let test_data: [u8; 16] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef,
        ];

        let (_, num) = util::take_u16(&test_data).unwrap();
        assert_eq!(num, 0x0123);
        let (_, num) = util::take_u32(&test_data).unwrap();
        assert_eq!(num, 0x01234567);
        let (_, num) = util::take_u64(&test_data).unwrap();
        assert_eq!(num, 0x0123456789abcdef);
        let (_, num) = util::take_u128(&test_data).unwrap();
        assert_eq!(num, 0x0123456789abcdef0123456789abcdef);
    }

    #[test]
    fn test_to_bytes() {
        let mut dst16 = [0u8; 2];
        util::u16_to_bytes(0x0123, &mut dst16);
        assert_eq!(dst16, [0x01, 0x23]);

        let mut dst32 = [0u8; 4];
        util::u32_to_bytes(0x01234567, &mut dst32);
        assert_eq!(dst32, [0x01, 0x23, 0x45, 0x67]);

        let mut dst64 = [0u8; 8];
        util::u64_to_bytes(0x0123456789abcdef, &mut dst64);
        assert_eq!(dst64, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

        let mut dst128 = [0u8; 16];
        util::u128_to_bytes(0x0123456789abcdef0123456789abcdef, &mut dst128);
        assert_eq!(
            dst128,
            [
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef,
            ]
        );
    }
}
