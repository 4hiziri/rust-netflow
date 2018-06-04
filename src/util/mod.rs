use nom;
use nom::{be_u16, be_u32};

// TODO: check nom::IResult
named!(pub take_u16 <&[u8], nom::IResult<&[u8], u16>>, map!(take!(2), be_u16));
named!(pub take_u32 <&[u8], nom::IResult<&[u8], u32>>, map!(take!(4), be_u32));
