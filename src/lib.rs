use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

use flate2::Compression;
use std::io::{Read, Write};

use mozjpeg::{self, Compress, decompress::{DecompressConfig, MarkerData, DecompressStarted}, Decompress};

#[cfg(feature = "browser")]
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
base64 input and output(flate2)
*/

#[cfg(feature = "strings")]
#[cfg_attr(feature = "browser", wasm_bindgen)]
pub extern "C" fn zlib_decode(base_compressed: &str) -> String {
    let compressed_bytes = base64::decode(&base_compressed).unwrap();
    let mut d = ZlibDecoder::new(&compressed_bytes[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    return s;
}

#[cfg(feature = "strings")]
#[cfg_attr(feature = "browser", wasm_bindgen)]
pub extern "C" fn zlib_encode(base_raw: &str) -> String {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw.as_bytes())
        .expect("could not compress");
    let compressed_bytes = e.finish();
    let s = base64::encode(&compressed_bytes.unwrap());
    return s;
}

#[cfg(feature = "strings")]
#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn gzip_decode(base_compressed: &str) -> String {
    let compressed_bytes = base64::decode(&base_compressed).unwrap();
    let mut d = GzDecoder::new(&compressed_bytes[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    return s;
}

#[cfg(feature = "strings")]
#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn gzip_encode(base_raw: &str) -> String {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw.as_bytes())
        .expect("could not compress");
    let compressed_bytes = e.finish();
    let s = base64::encode(&compressed_bytes.unwrap());
    return s;
}

#[cfg(feature = "strings")]
#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn deflate_decode(base_compressed: &str) -> String {
    let compressed_bytes = base64::decode(&base_compressed).unwrap();
    let mut deflater = DeflateDecoder::new(&compressed_bytes[..]);
    let mut s = String::new();
    deflater.read_to_string(&mut s).unwrap();
    return s;
}

#[cfg(feature = "strings")]
#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn deflate_encode(base_raw: &str) -> String {
    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw.as_bytes()).unwrap();
    let bytes = e.finish().unwrap();
    let s = base64::encode(&bytes);
    return s;
}

/*
u8array input and output(flate2)
*/

#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn zlib_decode_raw(base_compressed: &[u8]) -> Vec<u8> {
    let mut d = ZlibDecoder::new(&base_compressed[..]);
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    return buffer;
}

#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn zlib_encode_raw(base_raw: &[u8]) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw).expect("could not compress");
    let compressed_bytes = e.finish();
    return compressed_bytes.unwrap();
}

#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn gzip_decode_raw(base_compressed: &[u8]) -> Vec<u8> {
    let mut d = GzDecoder::new(&base_compressed[..]);
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    return buffer;
}

#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn gzip_encode_raw(base_raw: &[u8]) -> Vec<u8> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw).expect("could not compress");
    let compressed_bytes = e.finish();
    return compressed_bytes.unwrap();
}

#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn deflate_decode_raw(base_compressed: &[u8]) -> Vec<u8> {
    let mut d = DeflateDecoder::new(&base_compressed[..]);
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    return buffer;
}

#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn deflate_encode_raw(base_raw: &[u8]) -> Vec<u8> {
    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw).unwrap();
    let compressed_bytes = e.finish();
    return compressed_bytes.unwrap();
}

/*
u8array input and output(mozjpeg)
*/

#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn mozjpeg_encode(base_raw: &[u8], width: usize, height: usize) -> Vec<u8> {
    let jpeg_bytes: Vec<u8> = Vec::new();

    std::panic::catch_unwind(|| {
        let mut comp:Compress = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);

        comp.set_size(width, height);
        comp.set_mem_dest();
        comp.start_compress();

        // replace with your image data
        let pixels: Vec<u8> = vec![0; width * height * 3];
        assert!(comp.write_scanlines(&pixels[..]));

        comp.finish_compress();
        jpeg_bytes = comp.data_to_vec()?;
        // write to file, etc.
        Ok(())
    })?;
    return jpeg_bytes;
}


#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn mozjpeg_decode(base_raw: &[u8], width: usize, height: usize) -> Vec<u8> {
    let jpeg_bytes: Vec<u8> = Vec::new();

    std::panic::catch_unwind(|| {
    let d:DecompressConfig = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS);
    let d:Decompress = d.from_mem(base_raw)?;
    d.width(); // FYI
    d.height();
    d.color_space() == mozjpeg::ColorSpace::JCS_YCbCr;
    for marker in d.markers() {}

    // rgb() enables conversiono
    let image: DecompressStarted = d.rgb()?;
    image.width();
    image.height();
    image.color_space() == mozjpeg::ColorSpace::JCS_RGB;

    jpeg_bytes = image.read_scanlines()?;
    assert!(image.finish_decompress());
    Ok(())
    })?;
    return jpeg_bytes;
}

/*
u8array input and output(oxipng)
*/




/*
u8array input and output(brotli)
*/
