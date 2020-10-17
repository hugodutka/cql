use crate::frame::{Frame, Opcode, Version};
use crate::types::{CQLLongString, CQLSerializable};

pub struct Query {
    pub text: String,
}
