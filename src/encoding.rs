//! Decoding/Encoding functions for the Geometry Dash's save file format

use std::{io::Read, error::Error};

use base64::engine::fast_portable;
use libflate::gzip;
use thiserror::Error;

pub type DecodingResult<T> = Result<T, DecodingError>;

#[derive(Error, Debug)]
pub enum DecodingError {
    #[error("Error when decoding data with Base64: {0}")]
    Base64(Box<dyn Error>),

    #[error("Error when decoding data with Gzip: {0}")]
    Gzip(Box<dyn Error>),
}

/// Decodes data from the Geometry Dash encoding format
///
/// This function applies the following decoding functions of the provided data:
/// XOR (With key of 11, 0xB) => Url safe Base64 => GZIP
///
/// # Errors
///
/// This function can error if it is unable to perform Base64 decoding or GZIP decoding
pub fn decode<T : AsRef<[u8]>>(data: T) -> DecodingResult<Vec<u8>> {
    let xor = data.as_ref()
        .iter()
        .map(|b| b ^ 11)
        // For some reason, I have null bytes at the end of the file sometimes
        .filter(|&b| b != 0)
        .collect::<Vec<u8>>();

    let b64 = match base64::decode_engine(
        xor,
        &fast_portable::FastPortable::from(&base64::alphabet::URL_SAFE, fast_portable::PAD),
    ) {
        Ok(v) => v,
        Err(e) => return Err(DecodingError::Base64(Box::new(e))),
    };

    let mut gzip = Vec::new();
    let mut decoder = match gzip::Decoder::new(&b64[..]) {
        Ok(v) => v,
        Err(e) => return Err(DecodingError::Gzip(Box::new(e))),
    };

    match decoder.read_to_end(&mut gzip) {
        Ok(_) => Ok(gzip),
        Err(e) => Err(DecodingError::Gzip(Box::new(e))),
    }
}
