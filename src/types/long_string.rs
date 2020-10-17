use super::CQLSerializable;

pub struct CQLLongString<'a> {
    pub string: &'a str,
}

impl CQLSerializable for CQLLongString<'_> {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![];

        let length = self.string.len() as i32;
        buf.extend_from_slice(&length.to_be_bytes());
        buf.extend_from_slice(&self.string.as_bytes()[..(length as usize)]);

        buf
    }
}
