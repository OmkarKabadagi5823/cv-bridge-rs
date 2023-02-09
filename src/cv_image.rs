use crate::utils::{
    image_encodings,
    image_encoding_ops,
};
use rosrust_msg::{
    std_msgs::Header,
    sensor_msgs::Image,
};
use opencv::prelude::*;
use std::error;

#[derive(Debug)]
pub struct CvImage {
    header: Header,
    height: usize,
    width: usize,
    encoding: String,
    data: Vec<u8>,
}

impl CvImage {
    pub fn from_imgmsg(image: Image) -> CvImage {
        
        CvImage {
            header: image.header,
            height: image.height as usize,
            width: image.width as usize,
            encoding: image.encoding,
            data: if image.is_bigendian == 0 {
                image.data
            } else {
                image_encoding_ops::from_be_to_le(&image.data)
            },
        }
    }

    pub fn from_vec(height: usize, width: usize, encoding: &str, image: Vec<u8>) -> CvImage {
        CvImage {
            header: Header::default(),
            height: height,
            width: width,
            encoding: encoding.to_string(),
            data: image,
        }
    }

    pub fn from_cvmat(mat: Mat, encoding: &str) -> CvImage {
        let (width, height) = (mat.cols(), mat.rows());
        let data = match mat.data_bytes() {
            Ok(data) => data,
            Err(_) => panic!("Failed to get data bytes from Mat")
        };

        CvImage {
            header: Header::default(),
            height: height as usize,
            width: width as usize,
            encoding: encoding.to_string(),
            data: data.to_vec(),
        }
    }

    pub fn into_imgmsg(self, is_bigendian: u8) -> Image {
        let step = self.width as u32 * image_encodings::get_num_channels(&self.encoding) as u32;
        
        Image {
            header: self.header,
            height: self.height as u32,
            width: self.width as u32,
            encoding: self.encoding,
            is_bigendian: is_bigendian,
            step: step,
            data: if is_bigendian == 0 {
                self.data
            } else {
                image_encoding_ops::from_le_to_be(&self.data)
            },
        }
    }

    pub fn to_cvimage(&mut self, desired_encoding: &str) -> Result<CvImage, Box<dyn error::Error>> {
        let src_mat = self.as_cvmat()?;
        let mut dst_mat = Mat::default();
        let src_enc = image_encodings::from_encstr_to_cvenc(&self.encoding)?;
        let dst_enc = image_encodings::from_encstr_to_cvenc(desired_encoding)?;

        opencv::imgproc::cvt_color(&src_mat, &mut dst_mat, image_encodings::get_conversion_code(src_enc, dst_enc)?, 0)?;

        Ok(CvImage::from_cvmat(dst_mat, desired_encoding))
    }

    pub fn as_cvmat(&mut self) -> Result<Mat, Box<dyn error::Error>> {
        let cvtype = image_encodings::from_encstr_to_cvtype(&self.encoding)?;
        let mat;
        unsafe {
            mat = Mat::new_rows_cols_with_data(
                self.height as i32,
                self.width as i32,
                cvtype,
                self.data.as_mut_ptr() as *mut _,
                opencv::core::Mat_AUTO_STEP
            )?;
        }

        Ok(mat)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn encoding(&self) -> &String {
        &self.encoding
    }
}