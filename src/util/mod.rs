use nom::{be_u16, be_u32};

// TODO: check nom::IResult
named!(pub take_u16 <&[u8], u16>, map!(take!(2), |i| be_u16(i).unwrap().1));
named!(pub take_u32 <&[u8], u32>, map!(take!(4), |i| be_u32(i).unwrap().1));
