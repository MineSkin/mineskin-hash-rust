#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use lodepng::FilterStrategy;
use napi::bindgen_prelude::{Buffer, ObjectFinalize};
use napi_derive::napi;
use sha2::{Digest, Sha256};

pub const SKIN_WIDTH: usize = 64;
pub const SKIN_HEIGHT: usize = 64;
pub const SKIN_CHANNELS: usize = 4;

const SKIN_DATA_LENGTH: usize = (SKIN_WIDTH * SKIN_HEIGHT * SKIN_CHANNELS);

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[napi]
pub struct ImageWithHashes {
    pub png: Buffer,
    pub minecraft_hash: Buffer,
    pub hash: Buffer,
    pub error: Option<String>,
}

#[napi]
pub fn encode_image(buffer: &[u8]) -> ImageWithHashes {
    encode_custom_image(buffer, SKIN_WIDTH, SKIN_HEIGHT)
}

#[napi]
pub fn encode_image_size(buffer: &[u8], width: u8, height: u8) -> ImageWithHashes {
    encode_custom_image(buffer, width as usize, height as usize)
}

// based on https://github.com/GeyserMC/global_api/blob/dev/1.0.2/native/skins/src/skin_convert/skin_codec.rs#L100
//#[napi]
pub fn encode_custom_image(buffer: &[u8], width: usize, height: usize) -> ImageWithHashes {
    let mut decoder = lodepng::Decoder::new();
    decoder.info_png_mut().interlace_method = 0; // should be 0 but just to be sure

    let decoded = decoder.decode(buffer);
    if decoded.is_err() {
        let error = decoded.unwrap_err();
        return ImageWithHashes {
            png: Buffer::from(vec![]),
            minecraft_hash: Buffer::from(vec![]),
            hash: Buffer::from(vec![]),
            error: Some(format!("{:?}", error)),
        };
    }
    let decoded1 = decoded.unwrap();
    let decoded_data = decoded1.bytes();

    let mut raw_data = vec![0; width * height * SKIN_CHANNELS];
    decoded_data.clone_into(&mut raw_data);

    // encode images like Minecraft does
    let mut encoder = lodepng::Encoder::new();
    encoder.set_auto_convert(false);
    encoder.info_png_mut().interlace_method = 0; // should be 0 but just to be sure

    let encoder_settings = encoder.settings_mut();
    encoder_settings.zlibsettings.set_level(4);
    encoder_settings.filter_strategy = FilterStrategy::ZERO;

    // println!("Encoding image with width: {}, height: {}", width, height);
    // println!("Raw data length: {}", raw_data.len());


    let result = encoder.encode(&raw_data, width, height);
    // println!("Result: {:?}", result);
    if result.is_err() {
        let error = result.unwrap_err();
        return ImageWithHashes {
            png: Buffer::from(vec![]),
            minecraft_hash: Buffer::from(vec![]),
            hash: Buffer::from(vec![]),
            error: Some(format!("{:?}", error)),
        };
    }
    let png = result.unwrap();

    let mut hasher = Sha256::new();

    hasher.update(&png);
    let minecraft_hash = hasher.finalize_reset();

    // make our own hash
    hasher.update(&raw_data);
    let hash = hasher.finalize();

    ImageWithHashes {
        png: Buffer::from(png.as_slice()),
        minecraft_hash: Buffer::from(minecraft_hash.as_slice()),
        hash: Buffer::from(hash.as_slice()),
        error: None,
    }
}
