#![deny(clippy::all)]

mod pixel_cleaner;

use lodepng::{FilterStrategy, RGBA};
use sha2::{Digest, Sha256};
use napi::{
    bindgen_prelude::{Buffer, ClassInstance, ObjectFinalize, This, Uint8Array, Unknown},
    Env, Property, Result,
};
use napi_derive::napi;
use crate::pixel_cleaner::clear_unused_pixels;

#[macro_use]
extern crate napi_derive;

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
    pub hex: String,
}

#[napi]
pub fn encode_image(buffer: &[u8], is_classic: bool) -> ImageWithHashes {
    encode_custom_image(buffer, is_classic, SKIN_WIDTH, SKIN_HEIGHT)
}

fn copy_slice(dst: &mut [u8], src: &[u8]) -> usize {
    let mut c = 0;
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d = *s;
        c += 1;
    }
    c
}

// based on https://github.com/GeyserMC/global_api/blob/dev/1.0.2/native/skins/src/skin_convert/skin_codec.rs#L100
//#[napi]
pub fn encode_custom_image(buffer: &[u8], is_classic: bool, width: usize, height: usize) -> ImageWithHashes {
    println!("Buffer length: {}", buffer.len());

    let mut decoder = lodepng::Decoder::new();
    decoder.info_png_mut().interlace_method = 0; // should be 0 but just to be sure

    let decoded = decoder.decode(buffer);
    let decoded1 = decoded.unwrap();
    let decoded_data = decoded1.bytes();

    let mut raw_data = vec![0; SKIN_DATA_LENGTH];
    println!("Raw length: {}", raw_data.len());
    copy_slice(&mut raw_data, &decoded_data);
    clear_unused_pixels(&mut raw_data, is_classic);




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

    let hex = write_hex(Buffer::from(minecraft_hash.as_slice()).as_ref());

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