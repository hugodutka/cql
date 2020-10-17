use super::{Frame, Opcode, Version};

/// Represents a startup frame. Can be converted to an actual Frame using `Frame::from`.
/// # Examples
///
/// ```no_run
/// let frame = Frame::from(Startup {})
/// ```
pub struct Startup;

impl From<Startup> for Frame {
    fn from(_: Startup) -> Frame {
        let mut body = vec![];

        let options = &0u16.to_be_bytes();
        body.extend_from_slice(options);

        Frame::new(Version::Request, Opcode::Startup, body)
    }
}
