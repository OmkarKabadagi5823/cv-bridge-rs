
use image::{ExtendedColorType};

pub fn from_str_to_color(encoding: &String) -> ExtendedColorType {
    match encoding.as_str() {
        "mono8" => ExtendedColorType::L8,
        "rgb8" => ExtendedColorType::Rgb8,
        "rgba8" => ExtendedColorType::Rgba8,
        "bgr8" => ExtendedColorType::Bgr8,
        "bgra8" => ExtendedColorType::Bgra8,
        "mono16" => ExtendedColorType::L16,
        "rgb16" => ExtendedColorType::Rgb16,
        "rgba16" => ExtendedColorType::Rgba16,
        _ => panic!("Unsupported encoding"),
    }
}

pub fn from_color_to_str(encoding: &ExtendedColorType) -> String {
    match encoding {
        ExtendedColorType::L8 => "mono8".to_string(),
        ExtendedColorType::Rgb8 => "rgb8".to_string(),
        ExtendedColorType::Rgba8 => "rgba8".to_string(),
        ExtendedColorType::Bgr8 => "bgr8".to_string(),
        ExtendedColorType::Bgra8 => "bgra8".to_string(),
        ExtendedColorType::L16 => "mono16".to_string(),
        ExtendedColorType::Rgb16 => "rgb16".to_string(),
        ExtendedColorType::Rgba16 => "rgba16".to_string(),
        _ => panic!("Unsupported encoding"),
    }
}

pub fn from_str_to_cvtype(encoding: &String) -> i32 {
    match encoding.as_str() {
        "mono8" => opencv::core::CV_8UC1,
        "rgb8" => opencv::core::CV_8UC3,
        "rgba8" => opencv::core::CV_8UC4,
        "bgr8" => opencv::core::CV_8UC3,
        "bgra8" => opencv::core::CV_8UC4,
        "mono16" => opencv::core::CV_16UC1,
        "rgb16" => opencv::core::CV_16UC3,
        "rgba16" => opencv::core::CV_16UC4,
        _ => panic!("Unsupported encoding"),
    }
}

pub fn from_cvtype_to_str(cvtype: i32) -> String {
    match cvtype {
        opencv::core::CV_8UC1 => "mono8".to_string(),
        opencv::core::CV_8UC3 => "rgb8".to_string(),
        opencv::core::CV_8UC4 => "rgba8".to_string(),
        opencv::core::CV_16UC1 => "mono16".to_string(),
        opencv::core::CV_16UC3 => "rgb16".to_string(),
        opencv::core::CV_16UC4 => "rgba16".to_string(),
        _ => panic!("Unsupported encoding"),
    }
}

pub fn from_color_to_cvtype(encoding: &ExtendedColorType) -> i32 {
    match encoding {
        ExtendedColorType::L8 => opencv::core::CV_8UC1,
        ExtendedColorType::Rgb8 => opencv::core::CV_8UC3,
        ExtendedColorType::Rgba8 => opencv::core::CV_8UC4,
        ExtendedColorType::Bgr8 => opencv::core::CV_8UC3,
        ExtendedColorType::Bgra8 => opencv::core::CV_8UC4,
        ExtendedColorType::L16 => opencv::core::CV_16UC1,
        ExtendedColorType::Rgb16 => opencv::core::CV_16UC3,
        ExtendedColorType::Rgba16 => opencv::core::CV_16UC4,
        _ => panic!("Unsupported encoding"),
    }
}