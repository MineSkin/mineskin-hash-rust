#![deny(clippy::all)]

use lodepng::{FilterStrategy, RGBA};
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
    pub png: Buffer,
    pub minecraft_hash: Buffer,
    pub hash: Buffer,
    pub hex: String,
}

#[napi]
pub fn encode_image(buffer: &[u8]) -> ImageWithHashes {
    encode_custom_image(buffer, SKIN_WIDTH as usize, SKIN_HEIGHT as usize)
}

// based on https://github.com/GeyserMC/global_api/blob/dev/1.0.2/native/skins/src/skin_convert/skin_codec.rs#L100
//#[napi]
pub fn encode_custom_image(buffer: &[u8], width: usize, height: usize) -> ImageWithHashes {
    let mut raw_data = vec![0; width * height * SKIN_CHANNELS as usize];
    buffer.iter().enumerate().for_each(|(i, byte)| {
        raw_data[i] = *byte;
    });

    // encode images like Minecraft does
    let mut encoder = lodepng::Encoder::new();
    encoder.set_auto_convert(false);
    encoder.info_png_mut().interlace_method = 0; // should be 0 but just to be sure

    let encoder_settings = encoder.settings_mut();
    encoder_settings.zlibsettings.set_level(4);
    encoder_settings.filter_strategy = FilterStrategy::ZERO;

    println!("Encoding image with width: {}, height: {}", width, height);
    println!("Raw data length: {}", raw_data.len());

    let result = encoder.encode(&raw_data, width, height);
    println!("Result: {:?}", result);
    let png = result.unwrap();

    let mut hasher = Sha256::new();

    hasher.update(&png);
    let minecraft_hash = hasher.finalize_reset();

    // make our own hash
    hasher.update(&raw_data);
    let hash = hasher.finalize();

    let hex = write_hex(minecraft_hash.as_ref());

    ImageWithHashes {
        png: Buffer::from(png.as_slice()),
        minecraft_hash: Buffer::from(minecraft_hash.as_slice()),
        hash: Buffer::from(hash.as_slice()),
        hex: hex
    }
}

fn write_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        core::fmt::write(&mut s, format_args!("{:02X}", byte)).unwrap();
    }
    s
}