use crate::frame::{Frame, Opcode, Version};
use crate::types::{CQLLongString, CQLSerializable};

pub struct Query {
    pub text: String,
}

impl From<Query> for Frame {
    fn from(query: Query) -> Frame {
        let mut body = vec![];

        body.extend(
            CQLLongString {
                string: query.text.as_str(),
            }
            .as_bytes(),
        );
        body.extend_from_slice(&[0, 0, 0]); // consistency set to 0, flags set to 0

        Frame::new(Version::Request, Opcode::Query, body)
    }
}
