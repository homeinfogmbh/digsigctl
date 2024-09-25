/// Helper trait to read types from a vector of bytes.
pub trait TryFromIo: Sized {
    fn try_from_io(bytes: Vec<u8>) -> std::io::Result<Self>;
}

/// Implementation of the above helper trait to allow to read strings from a bytes buffer.
impl TryFromIo for String {
    fn try_from_io(bytes: Vec<u8>) -> std::io::Result<Self> {
        Self::from_utf8(bytes).map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("encountered non-utf-8 data: {error:?}"),
            )
        })
    }
}
