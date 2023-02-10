//! This module contains the image encodings used by OpenCV
//! 
//! ## Supported encodings
//! * `mono8` - 8-bit single channel image
//! * `rgb8` - 8-bit 3 channel image
//! * `rgba8` - 8-bit 4 channel image
//! * `bgr8` - 8-bit 3 channel image
//! * `bgra8` - 8-bit 4 channel image
//! * `mono16` - 16-bit single channel image
//! * `rgb16` - 16-bit 3 channel image
//! * `rgba16` - 16-bit 4 channel image
//! * `bgr16` - 16-bit 3 channel image
//! * `bgra16` - 16-bit 4 channel image
//! * `bayer_rggb8` - 8-bit Bayer image
//! * `bayer_bggr8` - 8-bit Bayer image
//! * `bayer_gbrg8` - 8-bit Bayer image
//! * `bayer_grbg8` - 8-bit Bayer image
//! * `bayer_rggb16` - 16-bit Bayer image
//! * `bayer_bggr16` - 16-bit Bayer image
//! * `bayer_gbrg16` - 16-bit Bayer image
//! * `bayer_grbg16` - 16-bit Bayer image
//! * `yuv422` - 8-bit 2 channel image
//! * `yuv422_yuy2` - 8-bit 2 channel image

#[derive(Debug)]
pub enum Encoding {
    Gray,
    Rgb,
    Bgr,
    Rgba,
    Bgra,
    Yuv422,
    Yuv422yuy2,
    BayerRGGB, 
    BayerBGGR,
    BayerGBRG,
    BayerGRBG,
    Invalid
}

/// Returns the number of channels for the given encoding
/// 
/// ## Arguments
/// * `encoding` - The encoding to get the number of channels for (e.g. "rgb8")
/// 
/// ## Returns
/// The number of channels for the given encoding
pub fn get_num_channels(encoding: &str) -> usize {
    match encoding {
        "mono8" => 1,
        "rgb8" => 3,
        "rgba8" => 4,
        "bgr8" => 3,
        "bgra8" => 4,
        "mono16" => 1,
        "rgb16" => 3,
        "rgba16" => 4,
        "bgr16" => 3,
        "bgra16" => 4,
        "bayer_rggb8" => 1,
        "bayer_bggr8" => 1,
        "bayer_gbrg8" => 1,
        "bayer_grbg8" => 1,
        "bayer_rggb16" => 1,
        "bayer_bggr16" => 1,
        "bayer_gbrg16" => 1,
        "bayer_grbg16" => 1,
        "yuv422" => 2,
        "yuv422_yuy2" => 2,
        _ => 0
    }
}

/// Returns the bit depth for the given encoding
/// 
/// ## Arguments
/// * `encoding` - The encoding to get the bit depth for (e.g. "rgb8")
///                Note: The bit depth is the number of bits per channel
/// 
/// ## Returns
/// The bit depth for the given encoding
pub fn get_bit_depth(encoding: &str) -> u8 {
    match encoding {
        "mono8" => 8,
        "rgb8" => 8,
        "rgba8" => 8,
        "bgr8" => 8,
        "bgra8" => 8,
        "mono16" => 16,
        "rgb16" => 16,
        "rgba16" => 16,
        "bgr16" => 16,
        "bgra16" => 16,
        "bayer_rggb8" => 8,
        "bayer_bggr8" => 8,
        "bayer_gbrg8" => 8,
        "bayer_grbg8" => 8,
        "bayer_rggb16" => 16,
        "bayer_bggr16" => 16,
        "bayer_gbrg16" => 16,
        "bayer_grbg16" => 16,
        "yuv422" => 8,
        "yuv422_yuy2" => 8,
        _ => 0
    }
}

/// Returns the scaling factor when converting between encodings with
/// different bit depths
/// 
/// ## Arguments
/// * `src_encoding` - The source encoding
/// * `dst_encoding` - The destination encoding
/// 
/// ## Returns
/// The scaling factor when converting between encodings with different bit depths
pub fn get_scaling_factor(src_encoding: &str, dst_encoding: &str) -> f64 {
    let src_depth = get_bit_depth(src_encoding) as i32;
    let dst_depth = get_bit_depth(dst_encoding) as i32;

    if src_depth == 0 || dst_depth == 0 {
        return 0.0;
    }

    f64::powi(2.0, dst_depth - src_depth) as f64
}

/// Returns the OpenCV type for the given encoding
/// 
/// ## Arguments
/// * `encoding` - The encoding to get the OpenCV type for (e.g. "rgb8")
/// 
/// ## Returns
/// The OpenCV type for the given encoding or an error if the encoding is invalid
/// (eg. opencv::core::CV_8UC3)
pub fn from_encstr_to_cvtype(encoding: &str) -> Result<i32, String> {
    match encoding {
        "mono8" => Ok(opencv::core::CV_8UC1),
        "rgb8" => Ok(opencv::core::CV_8UC3),
        "rgba8" => Ok(opencv::core::CV_8UC4),
        "bgr8" => Ok(opencv::core::CV_8UC3),
        "bgra8" => Ok(opencv::core::CV_8UC4),
        "mono16" => Ok(opencv::core::CV_16UC1),
        "rgb16" => Ok(opencv::core::CV_16UC3),
        "rgba16" => Ok(opencv::core::CV_16UC4),
        "bgr16" => Ok(opencv::core::CV_16UC3),
        "bgra16" => Ok(opencv::core::CV_16UC4),
        "bayer_rggb8" => Ok(opencv::core::CV_8UC1),
        "bayer_bggr8" => Ok(opencv::core::CV_8UC1),
        "bayer_gbrg8" => Ok(opencv::core::CV_8UC1),
        "bayer_grbg8" => Ok(opencv::core::CV_8UC1),
        "bayer_rggb16" => Ok(opencv::core::CV_16UC1),
        "bayer_bggr16" => Ok(opencv::core::CV_16UC1),
        "bayer_gbrg16" => Ok(opencv::core::CV_16UC1),
        "bayer_grbg16" => Ok(opencv::core::CV_16UC1),
        "yuv422" => Ok(opencv::core::CV_8UC2),
        "yuv422_yuy2" => Ok(opencv::core::CV_8UC2),
        _ => Err(format!("Unsupported encoding type: {}", encoding))
    }
}

/// Returns the OpenCV encoding for the given encoding
/// 
/// ## Arguments
/// * `encoding` - The encoding to get the OpenCV encoding for (e.g. "rgb8")
/// 
/// ## Returns
/// The OpenCV encoding for the given encoding or an error if the encoding is invalid
pub fn from_encstr_to_cvenc(encoding: &str) -> Result<Encoding, String> {
    match encoding {
        "mono8" => Ok(Encoding::Gray),
        "rgb8" => Ok(Encoding::Rgb),
        "rgba8" => Ok(Encoding::Rgba),
        "bgr8" => Ok(Encoding::Bgr),
        "bgra8" => Ok(Encoding::Bgra),
        "mono16" => Ok(Encoding::Gray),
        "rgb16" => Ok(Encoding::Rgb),
        "rgba16" => Ok(Encoding::Rgba),
        "bgr16" => Ok(Encoding::Bgr),
        "bgra16" => Ok(Encoding::Bgra),
        "bayer_rggb8" => Ok(Encoding::BayerRGGB),
        "bayer_bggr8" => Ok(Encoding::BayerBGGR),
        "bayer_gbrg8" => Ok(Encoding::BayerGBRG),
        "bayer_grbg8" => Ok(Encoding::BayerGRBG),
        "bayer_rggb16" => Ok(Encoding::BayerRGGB),
        "bayer_bggr16" => Ok(Encoding::BayerBGGR),
        "bayer_gbrg16" => Ok(Encoding::BayerGBRG),
        "bayer_grbg16" => Ok(Encoding::BayerGRBG),
        "yuv422" => Ok(Encoding::Yuv422),
        "yuv422_yuy2" => Ok(Encoding::Yuv422yuy2),
        _ => Err(format!("Unsupported encoding type: {}", encoding))
    }
}

/// Returns the encoding for the given OpenCV encoding and bit depth
/// 
/// ## Arguments
/// * `cvenc` - The OpenCV encoding to get the encoding for
/// * `cvdepth` - The OpenCV bit depth to get the encoding for
///          (0 = unsigned 8-bit, 1 = signed 8-bit, 2 = unsigned 16-bit, 3 = signed 16-bit)
pub fn from_cvenc_to_encstr(cvenc: Encoding, cvdepth: i32) -> Result<String, String> {
    match (cvenc, cvdepth) {
        (Encoding::Gray, 0) => Ok("mono8".to_string()),
        (Encoding::Gray, 1) => Ok("mono8".to_string()),
        (Encoding::Gray, 2) => Ok("mono16".to_string()),
        (Encoding::Gray, 3) => Ok("mono16".to_string()),
        (Encoding::Rgb, 0) => Ok("rgb8".to_string()),
        (Encoding::Rgb, 1) => Ok("rgb8".to_string()),
        (Encoding::Rgb, 2) => Ok("rgb16".to_string()),
        (Encoding::Rgb, 3) => Ok("rgb16".to_string()),
        (Encoding::Rgba, 0) => Ok("rgba8".to_string()),
        (Encoding::Rgba, 1) => Ok("rgba8".to_string()),
        (Encoding::Rgba, 2) => Ok("rgba16".to_string()),
        (Encoding::Rgba, 3) => Ok("rgba16".to_string()),
        (Encoding::Bgr, 0) => Ok("bgr8".to_string()),
        (Encoding::Bgr, 1) => Ok("bgr8".to_string()),
        (Encoding::Bgr, 2) => Ok("bgr16".to_string()),
        (Encoding::Bgr, 3) => Ok("bgr16".to_string()),
        (Encoding::Bgra, 0) => Ok("bgra8".to_string()),
        (Encoding::Bgra, 1) => Ok("bgra8".to_string()),
        (Encoding::Bgra, 2) => Ok("bgra16".to_string()),
        (Encoding::Bgra, 3) => Ok("bgra16".to_string()),
        (Encoding::BayerRGGB, 0) => Ok("bayer_rggb8".to_string()),
        (Encoding::BayerRGGB, 1) => Ok("bayer_rggb8".to_string()),
        (Encoding::BayerRGGB, 2) => Ok("bayer_rggb16".to_string()),
        (Encoding::BayerRGGB, 3) => Ok("bayer_rggb16".to_string()),
        (Encoding::BayerBGGR, 0) => Ok("bayer_bggr8".to_string()),
        (Encoding::BayerBGGR, 1) => Ok("bayer_bggr8".to_string()),
        (Encoding::BayerBGGR, 2) => Ok("bayer_bggr16".to_string()),
        (Encoding::BayerBGGR, 3) => Ok("bayer_bggr16".to_string()),
        (Encoding::BayerGBRG, 0) => Ok("bayer_gbrg8".to_string()),
        (Encoding::BayerGBRG, 1) => Ok("bayer_gbrg8".to_string()),
        (Encoding::BayerGBRG, 2) => Ok("bayer_gbrg16".to_string()),
        (Encoding::BayerGBRG, 3) => Ok("bayer_gbrg16".to_string()),
        (Encoding::BayerGRBG, 0) => Ok("bayer_grbg8".to_string()),
        (Encoding::BayerGRBG, 1) => Ok("bayer_grbg8".to_string()),
        (Encoding::BayerGRBG, 2) => Ok("bayer_grbg16".to_string()),
        (Encoding::BayerGRBG, 3) => Ok("bayer_grbg16".to_string()),
        (Encoding::Yuv422, 0) => Ok("yuv422".to_string()),
        (Encoding::Yuv422yuy2, 0) => Ok("yuv422_yuy2".to_string()),
        _ => Err(format!("Unsupported encoding type"))

    }
}

/// Returns the conversion code for going from one color space to another
/// 
/// ## Arguments
/// * `src_encoding` - The source encoding
/// * `dst_encoding` - The destination encoding
/// 
/// ## Returns
/// * `Ok(i32)` - The conversion code (eg. opencv::imgproc::COLOR_RGB2GRAY)
pub fn get_conversion_code(src_encoding: Encoding, dst_encoding: Encoding) -> Result<i32, String>{
    match (&src_encoding, &dst_encoding) {
        (Encoding::Gray, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_GRAY2RGB),
        (Encoding::Gray, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_GRAY2BGR),
        (Encoding::Gray, Encoding::Rgba) => Ok(opencv::imgproc::COLOR_GRAY2RGBA),
        (Encoding::Gray, Encoding::Bgra) => Ok(opencv::imgproc::COLOR_GRAY2BGRA),
        (Encoding::Rgb, Encoding::Gray) => Ok(opencv::imgproc::COLOR_RGB2GRAY),
        (Encoding::Rgb, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_RGB2BGR),
        (Encoding::Rgb, Encoding::Rgba) => Ok(opencv::imgproc::COLOR_RGB2RGBA),
        (Encoding::Rgb, Encoding::Bgra) => Ok(opencv::imgproc::COLOR_RGB2BGRA),
        (Encoding::Bgr, Encoding::Gray) => Ok(opencv::imgproc::COLOR_BGR2GRAY),
        (Encoding::Bgr, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_BGR2RGB),
        (Encoding::Bgr, Encoding::Rgba) => Ok(opencv::imgproc::COLOR_BGR2RGBA),
        (Encoding::Bgr, Encoding::Bgra) => Ok(opencv::imgproc::COLOR_BGR2BGRA),
        (Encoding::Rgba, Encoding::Gray) => Ok(opencv::imgproc::COLOR_RGBA2GRAY),
        (Encoding::Rgba, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_RGBA2RGB),
        (Encoding::Rgba, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_RGBA2BGR),
        (Encoding::Rgba, Encoding::Bgra) => Ok(opencv::imgproc::COLOR_RGBA2BGRA),
        (Encoding::Bgra, Encoding::Gray) => Ok(opencv::imgproc::COLOR_BGRA2GRAY),
        (Encoding::Bgra, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_BGRA2RGB),
        (Encoding::Bgra, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_BGRA2BGR),
        (Encoding::Bgra, Encoding::Rgba) => Ok(opencv::imgproc::COLOR_BGRA2RGBA),
        (Encoding::BayerRGGB, Encoding::Gray) => Ok(opencv::imgproc::COLOR_BayerBG2GRAY),
        (Encoding::BayerRGGB, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_BayerBG2RGB),
        (Encoding::BayerRGGB, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_BayerBG2BGR),
        (Encoding::BayerBGGR, Encoding::Gray) => Ok(opencv::imgproc::COLOR_BayerRG2GRAY),
        (Encoding::BayerBGGR, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_BayerRG2RGB),
        (Encoding::BayerBGGR, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_BayerRG2BGR),
        (Encoding::BayerGBRG, Encoding::Gray) => Ok(opencv::imgproc::COLOR_BayerGR2GRAY),
        (Encoding::BayerGBRG, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_BayerGR2RGB),
        (Encoding::BayerGBRG, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_BayerGR2BGR),
        (Encoding::BayerGRBG, Encoding::Gray) => Ok(opencv::imgproc::COLOR_BayerGB2GRAY),
        (Encoding::BayerGRBG, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_BayerGB2RGB),
        (Encoding::BayerGRBG, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_BayerGB2BGR),
        (Encoding::Yuv422, Encoding::Gray) => Ok(opencv::imgproc::COLOR_YUV2GRAY_UYVY),
        (Encoding::Yuv422, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_YUV2RGB_UYVY),
        (Encoding::Yuv422, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_YUV2BGRA_UYVY),
        (Encoding::Yuv422, Encoding::Rgba) => Ok(opencv::imgproc::COLOR_YUV2RGBA_UYVY),
        (Encoding::Yuv422, Encoding::Bgra) => Ok(opencv::imgproc::COLOR_YUV2BGR_UYVY),
        (Encoding::Yuv422yuy2, Encoding::Gray) => Ok(opencv::imgproc::COLOR_YUV2GRAY_YUY2),
        (Encoding::Yuv422yuy2, Encoding::Rgb) => Ok(opencv::imgproc::COLOR_YUV2RGB_YUY2),
        (Encoding::Yuv422yuy2, Encoding::Bgr) => Ok(opencv::imgproc::COLOR_YUV2BGR_YUY2),
        (Encoding::Yuv422yuy2, Encoding::Rgba) => Ok(opencv::imgproc::COLOR_YUV2RGBA_YUY2),
        (Encoding::Yuv422yuy2, Encoding::Bgra) => Ok(opencv::imgproc::COLOR_YUV2BGRA_YUY2),
        _ => Err(format!("Unsupported conversion from {:?} to {:?}", src_encoding, dst_encoding)),
    }
}