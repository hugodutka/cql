use super::{Frame, Opcode, Version};

pub struct Startup;

impl From<Startup> for Frame {
    fn from(_: Startup) -> Frame {
        let mut body = vec![];

        let options = &0u16.to_be_bytes();
        body.extend_from_slice(options);

        Frame::new(Version::Request, Opcode::Startup, body)
    }
}
