use super::{Frame, Opcode, Version};
use crate::query::Query;
use crate::types::{CQLLongString, CQLSerializable};

impl From<Query> for Frame {
    fn from(query: Query) -> Frame {
        let mut body = vec![];

        body.extend(
            CQLLongString {
                string: query.text.as_str(),
            }
            .as_bytes(),
        );
        // hardcoded for now
        body.extend_from_slice(&[
            0, 0, // consistency
            0, // flags
        ]);

        Frame::new(Version::Request, Opcode::Query, body)
    }
}
