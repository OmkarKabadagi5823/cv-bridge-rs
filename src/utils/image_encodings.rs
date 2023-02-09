#[derive(Debug)]
pub enum Encoding {
    GRAY,
    RGB,
    BGR,
    RGBA,
    BGRA,
    YUV422,
    YUV422_YUY2,
    BAYER_RGGB, 
    BAYER_BGGR,
    BAYER_GBRG,
    BAYER_GRBG,
    INVALID
}

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

pub fn from_depthstr_to_int(depth: &str) -> Result<i32, String> {
    match depth {
        "8U" => Ok(0),
        "8S" => Ok(1),
        "16U" => Ok(2),
        "16S" => Ok(3),
        "32S" => Ok(4),
        "32F" => Ok(5),
        _ => Ok(6)
    }
}

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

pub fn from_encstr_to_cvenc(encoding: &str) -> Result<Encoding, String> {
    match encoding {
        "mono8" => Ok(Encoding::GRAY),
        "rgb8" => Ok(Encoding::RGB),
        "rgba8" => Ok(Encoding::RGBA),
        "bgr8" => Ok(Encoding::BGR),
        "bgra8" => Ok(Encoding::BGRA),
        "mono16" => Ok(Encoding::GRAY),
        "rgb16" => Ok(Encoding::RGB),
        "rgba16" => Ok(Encoding::RGBA),
        "bgr16" => Ok(Encoding::BGR),
        "bgra16" => Ok(Encoding::BGRA),
        "bayer_rggb8" => Ok(Encoding::BAYER_RGGB),
        "bayer_bggr8" => Ok(Encoding::BAYER_BGGR),
        "bayer_gbrg8" => Ok(Encoding::BAYER_GBRG),
        "bayer_grbg8" => Ok(Encoding::BAYER_GRBG),
        "bayer_rggb16" => Ok(Encoding::BAYER_RGGB),
        "bayer_bggr16" => Ok(Encoding::BAYER_BGGR),
        "bayer_gbrg16" => Ok(Encoding::BAYER_GBRG),
        "bayer_grbg16" => Ok(Encoding::BAYER_GRBG),
        "yuv422" => Ok(Encoding::YUV422),
        "yuv422_yuy2" => Ok(Encoding::YUV422_YUY2),
        _ => Err(format!("Unsupported encoding type: {}", encoding))
    }
}

pub fn from_cvenc_to_encstr(cvenc: Encoding, cvdepth: i32) -> Result<String, String> {
    match (cvenc, cvdepth) {
        (Encoding::GRAY, 0) => Ok("mono8".to_string()),
        (Encoding::GRAY, 1) => Ok("mono8".to_string()),
        (Encoding::GRAY, 2) => Ok("mono16".to_string()),
        (Encoding::GRAY, 3) => Ok("mono16".to_string()),
        (Encoding::RGB, 0) => Ok("rgb8".to_string()),
        (Encoding::RGB, 1) => Ok("rgb8".to_string()),
        (Encoding::RGB, 2) => Ok("rgb16".to_string()),
        (Encoding::RGB, 3) => Ok("rgb16".to_string()),
        (Encoding::RGBA, 0) => Ok("rgba8".to_string()),
        (Encoding::RGBA, 1) => Ok("rgba8".to_string()),
        (Encoding::RGBA, 2) => Ok("rgba16".to_string()),
        (Encoding::RGBA, 3) => Ok("rgba16".to_string()),
        (Encoding::BGR, 0) => Ok("bgr8".to_string()),
        (Encoding::BGR, 1) => Ok("bgr8".to_string()),
        (Encoding::BGR, 2) => Ok("bgr16".to_string()),
        (Encoding::BGR, 3) => Ok("bgr16".to_string()),
        (Encoding::BGRA, 0) => Ok("bgra8".to_string()),
        (Encoding::BGRA, 1) => Ok("bgra8".to_string()),
        (Encoding::BGRA, 2) => Ok("bgra16".to_string()),
        (Encoding::BGRA, 3) => Ok("bgra16".to_string()),
        (Encoding::BAYER_RGGB, 0) => Ok("bayer_rggb8".to_string()),
        (Encoding::BAYER_RGGB, 1) => Ok("bayer_rggb8".to_string()),
        (Encoding::BAYER_RGGB, 2) => Ok("bayer_rggb16".to_string()),
        (Encoding::BAYER_RGGB, 3) => Ok("bayer_rggb16".to_string()),
        (Encoding::BAYER_BGGR, 0) => Ok("bayer_bggr8".to_string()),
        (Encoding::BAYER_BGGR, 1) => Ok("bayer_bggr8".to_string()),
        (Encoding::BAYER_BGGR, 2) => Ok("bayer_bggr16".to_string()),
        (Encoding::BAYER_BGGR, 3) => Ok("bayer_bggr16".to_string()),
        (Encoding::BAYER_GBRG, 0) => Ok("bayer_gbrg8".to_string()),
        (Encoding::BAYER_GBRG, 1) => Ok("bayer_gbrg8".to_string()),
        (Encoding::BAYER_GBRG, 2) => Ok("bayer_gbrg16".to_string()),
        (Encoding::BAYER_GBRG, 3) => Ok("bayer_gbrg16".to_string()),
        (Encoding::BAYER_GRBG, 0) => Ok("bayer_grbg8".to_string()),
        (Encoding::BAYER_GRBG, 1) => Ok("bayer_grbg8".to_string()),
        (Encoding::BAYER_GRBG, 2) => Ok("bayer_grbg16".to_string()),
        (Encoding::BAYER_GRBG, 3) => Ok("bayer_grbg16".to_string()),
        (Encoding::YUV422, 0) => Ok("yuv422".to_string()),
        (Encoding::YUV422_YUY2, 0) => Ok("yuv422_yuy2".to_string()),
        _ => Err(format!("Unsupported encoding type"))

    }
}

pub fn get_conversion_code(src_encoding: Encoding, dst_encoding: Encoding) -> Result<i32, String>{
    match (&src_encoding, &dst_encoding) {
        (Encoding::GRAY, Encoding::RGB) => Ok(opencv::imgproc::COLOR_GRAY2RGB),
        (Encoding::GRAY, Encoding::BGR) => Ok(opencv::imgproc::COLOR_GRAY2BGR),
        (Encoding::GRAY, Encoding::RGBA) => Ok(opencv::imgproc::COLOR_GRAY2RGBA),
        (Encoding::GRAY, Encoding::BGRA) => Ok(opencv::imgproc::COLOR_GRAY2BGRA),
        (Encoding::RGB, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_RGB2GRAY),
        (Encoding::RGB, Encoding::BGR) => Ok(opencv::imgproc::COLOR_RGB2BGR),
        (Encoding::RGB, Encoding::RGBA) => Ok(opencv::imgproc::COLOR_RGB2RGBA),
        (Encoding::RGB, Encoding::BGRA) => Ok(opencv::imgproc::COLOR_RGB2BGRA),
        (Encoding::BGR, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_BGR2GRAY),
        (Encoding::BGR, Encoding::RGB) => Ok(opencv::imgproc::COLOR_BGR2RGB),
        (Encoding::BGR, Encoding::RGBA) => Ok(opencv::imgproc::COLOR_BGR2RGBA),
        (Encoding::BGR, Encoding::BGRA) => Ok(opencv::imgproc::COLOR_BGR2BGRA),
        (Encoding::RGBA, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_RGBA2GRAY),
        (Encoding::RGBA, Encoding::RGB) => Ok(opencv::imgproc::COLOR_RGBA2RGB),
        (Encoding::RGBA, Encoding::BGR) => Ok(opencv::imgproc::COLOR_RGBA2BGR),
        (Encoding::RGBA, Encoding::BGRA) => Ok(opencv::imgproc::COLOR_RGBA2BGRA),
        (Encoding::BGRA, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_BGRA2GRAY),
        (Encoding::BGRA, Encoding::RGB) => Ok(opencv::imgproc::COLOR_BGRA2RGB),
        (Encoding::BGRA, Encoding::BGR) => Ok(opencv::imgproc::COLOR_BGRA2BGR),
        (Encoding::BGRA, Encoding::RGBA) => Ok(opencv::imgproc::COLOR_BGRA2RGBA),
        (Encoding::BAYER_RGGB, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_BayerBG2GRAY),
        (Encoding::BAYER_RGGB, Encoding::RGB) => Ok(opencv::imgproc::COLOR_BayerBG2RGB),
        (Encoding::BAYER_RGGB, Encoding::BGR) => Ok(opencv::imgproc::COLOR_BayerBG2BGR),
        (Encoding::BAYER_BGGR, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_BayerRG2GRAY),
        (Encoding::BAYER_BGGR, Encoding::RGB) => Ok(opencv::imgproc::COLOR_BayerRG2RGB),
        (Encoding::BAYER_BGGR, Encoding::BGR) => Ok(opencv::imgproc::COLOR_BayerRG2BGR),
        (Encoding::BAYER_GBRG, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_BayerGR2GRAY),
        (Encoding::BAYER_GBRG, Encoding::RGB) => Ok(opencv::imgproc::COLOR_BayerGR2RGB),
        (Encoding::BAYER_GBRG, Encoding::BGR) => Ok(opencv::imgproc::COLOR_BayerGR2BGR),
        (Encoding::BAYER_GRBG, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_BayerGB2GRAY),
        (Encoding::BAYER_GRBG, Encoding::RGB) => Ok(opencv::imgproc::COLOR_BayerGB2RGB),
        (Encoding::BAYER_GRBG, Encoding::BGR) => Ok(opencv::imgproc::COLOR_BayerGB2BGR),
        (Encoding::YUV422, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_YUV2GRAY_UYVY),
        (Encoding::YUV422, Encoding::RGB) => Ok(opencv::imgproc::COLOR_YUV2RGB_UYVY),
        (Encoding::YUV422, Encoding::BGR) => Ok(opencv::imgproc::COLOR_YUV2BGRA_UYVY),
        (Encoding::YUV422, Encoding::RGBA) => Ok(opencv::imgproc::COLOR_YUV2RGBA_UYVY),
        (Encoding::YUV422, Encoding::BGRA) => Ok(opencv::imgproc::COLOR_YUV2BGR_UYVY),
        (Encoding::YUV422_YUY2, Encoding::GRAY) => Ok(opencv::imgproc::COLOR_YUV2GRAY_YUY2),
        (Encoding::YUV422_YUY2, Encoding::RGB) => Ok(opencv::imgproc::COLOR_YUV2RGB_YUY2),
        (Encoding::YUV422_YUY2, Encoding::BGR) => Ok(opencv::imgproc::COLOR_YUV2BGR_YUY2),
        (Encoding::YUV422_YUY2, Encoding::RGBA) => Ok(opencv::imgproc::COLOR_YUV2RGBA_YUY2),
        (Encoding::YUV422_YUY2, Encoding::BGRA) => Ok(opencv::imgproc::COLOR_YUV2BGRA_YUY2),
        _ => Err(format!("Unsupported conversion from {:?} to {:?}", src_encoding, dst_encoding)),
    }
}