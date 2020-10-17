use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

mod frame_query;
mod frame_startup;

pub use frame_startup::Startup;

lazy_static! {
    static ref STREAM_ID: Arc<Mutex<i16>> = Arc::new(Mutex::new(0));
}

pub struct Frame {
    version: Version,
    opcode: Opcode,
    stream: i16,
    body: Vec<u8>,
}

impl Frame {
    pub fn new(version: Version, opcode: Opcode, body: Vec<u8>) -> Frame {
        let mut stream_id = STREAM_ID.lock().unwrap();
        let stream = *stream_id;
        if stream == i16::MAX {
            *stream_id = 0;
        } else {
            *stream_id += 1;
        }

        Frame {
            version,
            opcode,
            stream,
            body,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![];

        let version = self.version.as_byte();
        let flags = 0; // hardcoded for now
        let stream = &self.stream.to_be_bytes();
        let opcode = self.opcode.as_byte();
        let length = &(self.body.len() as u32).to_be_bytes();
        let body = &self.body[..(self.body.len() as u32) as usize];

        buf.push(version);
        buf.push(flags);
        buf.extend_from_slice(stream);
        buf.push(opcode);
        buf.extend_from_slice(length);
        buf.extend_from_slice(body);

        buf
    }
}

pub enum Version {
    Request,
}

impl Version {
    pub fn as_byte(&self) -> u8 {
        match self {
            Version::Request => 0x04,
        }
    }
}

pub enum Opcode {
    Startup,
    Query,
}

impl Opcode {
    pub fn as_byte(&self) -> u8 {
        match self {
            Opcode::Startup => 0x01,
            Opcode::Query => 0x07,
        }
    }
}
