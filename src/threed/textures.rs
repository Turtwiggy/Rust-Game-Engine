use image;
use image::GenericImage;
use image::{open, DynamicImage};
use image::DynamicImage::*;

use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

/// utility function for loading a 2D texture from file
/// ---------------------------------------------------
#[allow(dead_code)]
pub fn loadTexture(gl: &gl::Gl, path: &str) -> u32 {

    unsafe{
    let mut textureID = 0;

    gl.GenTextures(1, &mut textureID);
    let img = image::open(&Path::new(path)).expect("Texture failed to load");
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
    };

    let data = img.raw_pixels();

    gl.BindTexture(gl::TEXTURE_2D, textureID);
    gl.TexImage2D(
        gl::TEXTURE_2D,
        0,
        format as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        format,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );
    gl.GenerateMipmap(gl::TEXTURE_2D);

    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl.TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR as i32,
    );
    gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    textureID
    }
}
