use crate::utils::{
    image_encodings,
    image_encoding_ops,
};
use rosrust_msg::{
    std_msgs::Header,
    sensor_msgs::Image,
};
use image::{DynamicImage, ImageBuffer, ExtendedColorType, GenericImageView};
use opencv::prelude::*;

#[derive(Debug)]
enum BufferMutPtr {
    Mut8(*mut u8),
    Mut16(*mut u16),
}

#[derive(Debug)]
pub struct CvImage {
    header: Header,
    image: DynamicImage,
}

impl CvImage {
    pub fn from_imgmsg(image: Image) -> CvImage {
        CvImage {
            header: image.header,
            image: CvImage::from_vec(image.width, image.height, image.encoding, image.data).image,
        }
    }

    pub fn from_cvmat(mat: Mat) -> CvImage {
        let (width, height) = (mat.cols(), mat.rows());
        let encoding = image_encodings::from_cvtype_to_str(mat.typ());
        let data = match mat.data_bytes() {
            Ok(data) => data.to_vec(),
            Err(_) => panic!("Could not get data from Mat"),
        };
        
        CvImage { 
            header: Header::default(), 
            image: CvImage::from_vec(width as u32, height as u32, encoding, data).image
        }
    }

    pub fn from_vec(width: u32, height: u32, encoding: String, data: Vec<u8>) -> CvImage {
        match image_encodings::from_str_to_color(&encoding) {
            ExtendedColorType::L8 => {
                let img = ImageBuffer::from_vec(width, height, data).unwrap();
                CvImage {
                    header: Header::default(),
                    image: DynamicImage::ImageLuma8(img),
                }
            }
            ExtendedColorType::Rgb8 => {
                let img = ImageBuffer::from_vec(width, height, data).unwrap();
                CvImage {
                    header: Header::default(),
                    image: DynamicImage::ImageRgb8(img),
                }
            }
            ExtendedColorType::Rgba8 => {
                let img = ImageBuffer::from_vec(width, height, data).unwrap();
                CvImage {
                    header: Header::default(),
                    image: DynamicImage::ImageRgba8(img),
                }
            }
            ExtendedColorType::Bgr8 => {
                let img = ImageBuffer::from_vec(width, height, data).unwrap();
                CvImage {
                    header: Header::default(),
                    image: DynamicImage::ImageRgb8(img),
                }
            }
            ExtendedColorType::Bgra8 => {
                let img = ImageBuffer::from_vec(width, height, data).unwrap();
                CvImage {
                    header: Header::default(),
                    image: DynamicImage::ImageRgba8(img),
                }
            }
            ExtendedColorType::L16 => {
                let img = ImageBuffer::from_vec(width, height, image_encoding_ops::from_u8_to_u16(&data, false)).unwrap();
                CvImage {
                    header: Header::default(),
                    image: DynamicImage::ImageLuma16(img),
                }
            }
            ExtendedColorType::Rgb16 => {
                let img = ImageBuffer::from_vec(width, height, image_encoding_ops::from_u8_to_u16(&data, false)).unwrap();
                CvImage {
                    header: Header::default(),
                    image: DynamicImage::ImageRgb16(img),
                }
            }
            ExtendedColorType::Rgba16 => {
                let img = ImageBuffer::from_vec(width, height, image_encoding_ops::from_u8_to_u16(&data, false)).unwrap();
                CvImage {
                    header: Header::default(),
                    image: DynamicImage::ImageRgba16(img),
                }
            }
            _ => {
                panic!("Unsupported encoding");
            }
        }
    }

    pub fn as_cvmat(&mut self, desired_encoding: String) -> Mat {
        let image_buffer_ptr = match desired_encoding.as_str() {
            "mono8" => BufferMutPtr::Mut8(self.image.as_mut_luma8().unwrap().as_mut_ptr()),
            "rgb8" => BufferMutPtr::Mut8(self.image.as_mut_rgb8().unwrap().as_mut_ptr()),
            "rgba8" => BufferMutPtr::Mut8(self.image.as_mut_rgba8().unwrap().as_mut_ptr()),
            "bgr8" => BufferMutPtr::Mut8(self.image.as_mut_rgb8().unwrap().as_mut_ptr()),
            "bgra8" => BufferMutPtr::Mut8(self.image.as_mut_rgba8().unwrap().as_mut_ptr()),
            "mono16" => BufferMutPtr::Mut16(self.image.as_mut_luma16().unwrap().as_mut_ptr()),
            "rgb16" => BufferMutPtr::Mut16(self.image.as_mut_rgb16().unwrap().as_mut_ptr()),
            "rgba16" => BufferMutPtr::Mut16(self.image.as_mut_rgba16().unwrap().as_mut_ptr()),
            _ => panic!("Unsupported encoding"),
        };
        
        let mat;
        if let BufferMutPtr::Mut8(inner_ptr) =  image_buffer_ptr {
            unsafe {
                mat = Mat::new_rows_cols_with_data(
                    self.image.height() as i32,
                    self.image.width() as i32,
                    image_encodings::from_str_to_cvtype(&desired_encoding),
                    inner_ptr as *mut _,
                    opencv::core::Mat_AUTO_STEP
                ).unwrap();
            }
    
            mat   
        }
        else if let BufferMutPtr::Mut16(inner_ptr) =  image_buffer_ptr {
            unsafe {
                mat = Mat::new_rows_cols_with_data(
                    self.image.height() as i32,
                    self.image.width() as i32,
                    image_encodings::from_str_to_cvtype(&desired_encoding),
                    inner_ptr as *mut _,
                    opencv::core::Mat_AUTO_STEP
                ).unwrap();
            }
    
            mat   
        }
        else {
            panic!("Unsupported encoding");
        }
    }

    pub fn as_image(&self) -> &DynamicImage {
        &self.image
    }

    pub fn as_mut_image(&mut self) -> &mut DynamicImage {
        &mut self.image
    }

    pub fn into_imgmsg(self) -> Image {
        let color = self.image.color();
        let (width, height) = self.image.dimensions();
        let step = width * color.bytes_per_pixel() as u32;

        let encoding = image_encodings::from_color_to_str(&color.into());
        let data = self.image.as_bytes().into();

        Image {
            header: self.header,
            height: height as u32,
            width: width as u32,
            encoding,
            is_bigendian: 0,
            step: step as u32,
            data,
        }
    }
}