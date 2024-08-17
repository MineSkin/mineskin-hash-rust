#![deny(clippy::all)]

use lodepng::FilterStrategy;
use sha2::{Digest, Sha256};
use napi::{
    bindgen_prelude::{Buffer, ClassInstance, ObjectFinalize, This, Uint8Array, Unknown},
    Env, Property, Result,
};
use napi_derive::napi;

#[macro_use]
extern crate napi_derive;

pub const SKIN_WIDTH: u8 = 64;
pub const SKIN_HEIGHT: u8 = 64;
pub const SKIN_CHANNELS: u8 = 4;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub struct ImageWithHashes {
    pub png: Uint8Array,
    pub minecraft_hash: Uint8Array,
    pub hash: Uint8Array,
}

#[napi]
pub fn encode_image(raw_data: &[u8]) -> ImageWithHashes {
    encode_custom_image(raw_data, SKIN_WIDTH, SKIN_HEIGHT)
}

//#[napi]
pub fn encode_custom_image(raw_data: &[u8], width: u8, height: u8) -> ImageWithHashes {
    // encode images like Minecraft does
    let mut encoder = lodepng::Encoder::new();
    encoder.set_auto_convert(false);
    encoder.info_png_mut().interlace_method = 0; // should be 0 but just to be sure

    let mut encoder_settings = encoder.settings_mut();
    encoder_settings.zlibsettings.set_level(4);
    encoder_settings.filter_strategy = FilterStrategy::ZERO;

    let png = encoder.encode(raw_data, width as usize, height as usize).unwrap();

    let mut hasher = Sha256::new();

    hasher.update(&png);
    let minecraft_hash = hasher.finalize_reset();

    // make our own hash
    hasher.update(raw_data);
    let hash = hasher.finalize();

    ImageWithHashes {
        png: Uint8Array::from(png.as_slice()),
        minecraft_hash: Uint8Array::from(minecraft_hash.as_slice()),
        hash: Uint8Array::from(hash.as_slice()),
    }
}