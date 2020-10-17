mod long_string;

pub trait CQLSerializable {
    fn as_bytes(&self) -> Vec<u8>;
}

pub use long_string::CQLLongString;
