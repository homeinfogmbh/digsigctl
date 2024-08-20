pub trait TryFromIo: Sized {
    fn try_from_io(bytes: Vec<u8>) -> std::io::Result<Self>;
}

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
