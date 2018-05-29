use nom::be_u16;

named!(netflowfield <&[u8], TypeLengthField>,
       dbg!(map!(count!(map!(take!(2), be_u16), 2),
                 |v: Vec<_>| TypeLengthField::new(v[0].clone().unwrap().1, v[1].clone().unwrap().1))));

#[derive(Debug, Clone, Copy)]
pub struct TypeLengthField {
    pub type_id: u16,
    pub length: u16,
}

impl TypeLengthField {
    pub fn new(type_id: u16, length: u16) -> TypeLengthField {
        TypeLengthField {
            type_id: type_id,
            length: length,
        }
    }

    pub fn take_from(count: usize, data: &[u8]) -> Result<(&[u8], Vec<TypeLengthField>), ()> {
        // TODO: define Error type
        let mut rest = data;
        let mut field_vec = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (next, field) = netflowfield(&rest).unwrap();
            field_vec.push(field);
            rest = next;
        }

        Ok((rest, field_vec))
    }
}
