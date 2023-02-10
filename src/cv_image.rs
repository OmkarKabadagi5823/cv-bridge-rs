//! cv_image module contains CvImage struct and its methods.
//! CvImage wraps the image array and its metadata and acts as
//! a bridge between the `sensor_msgs::Image` message and `cv::Mat`

use opencv::prelude::*;
use rosrust_msg::{
    std_msgs::Header,
    sensor_msgs::Image,
};
use std::error::Error;

use crate::utils::{
    image_encodings,
    image_byteorder_ops,
};

#[derive(Debug)]
pub enum DataContainer {
    VecU8(Vec<u8>),
    VecI8(Vec<i8>),
    VecU16(Vec<u16>),
    VecI16(Vec<i16>),
    VecI32(Vec<u32>),
    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
}

#[derive(Debug)]
pub struct CvImage {
    header: Header,
    height: usize,
    width: usize,
    encoding: String,
    data: DataContainer,
}

impl CvImage {
    /// Constructs a new `CvImage` from a `sensor_msgs::Image` message.
    /// 
    /// # Example
    /// ```
    /// let image = rosrust_msg::sensor_msgs::Image::default();
    /// // set the image data
    /// let cv_image = CvImage::from_imgmsg(image).unwrap();
    /// ```
    /// 
    /// ## Arguments
    /// * `image` - `rosrust_msg::sensor_msgs::Image` message 
    /// 
    /// ## Returns
    /// * `CvImage` object
    pub fn from_imgmsg(image: Image) -> Result<CvImage, Box<dyn Error>> {
        let bit_depth = image_encodings::get_bit_depth(image.encoding.as_str());
       
        let data = match bit_depth {
            8 => DataContainer::VecU8(image.data),
            16 => DataContainer::VecU8(match image.is_bigendian {
                0 => image.data,
                1 => image_byteorder_ops::from_be_to_le(&image.data),
                _ => Err(format!("Unsupported endianness [endianness: {}]", image.is_bigendian))?
            }),
            _ => Err(format!("Unsupported bit depth for container [bit depth: {}]", bit_depth))?
        };

        Ok(CvImage {
            header: image.header,
            height: image.height as usize,
            width: image.width as usize,
            encoding: image.encoding,
            data: data
        })
    }

    /// Constructs a new `CvImage` from a `cv::Mat` object.
    /// 
    /// # Example
    /// ```
    /// let mat = opencv::core::Mat::default();
    /// // set the image data
    /// let cv_image = CvImage::from_cvmat(mat, "bgr8").unwrap();
    /// ```
    /// 
    /// ## Arguments
    /// * `mat` - `opencv::core::Mat` object
    /// * `encoding` - Encoding of the image. Note that `Mat` does not
    ///                 contain any metadata about the image encoding, so it must be
    ///                 tracked by the user.
    /// 
    /// ## Returns
    /// * `CvImage` object
    pub fn from_cvmat(mat: Mat, encoding: &str) -> Result<CvImage, Box<dyn Error>> {
        let (width, height) = (mat.cols(), mat.rows());
        let bit_depth = image_encodings::get_bit_depth(encoding);

        let data = match mat.data_bytes() {
            Ok(data) => data,
            Err(_) => Err(format!("Could not get data from Mat"))?
        };
        
        let data = match bit_depth {
            8 => DataContainer::VecU8(data.to_vec()),
            16 => DataContainer::VecU8(data.to_vec()),
            _ => Err(format!("Unsupported bit depth for container [bit depth: {}]", bit_depth))?
        };

        Ok(CvImage {
            header: Header::default(),
            height: height as usize,
            width: width as usize,
            encoding: encoding.to_string(),
            data: data,
        })
    }

    /// Converts the `CvImage` to a `sensor_msgs::Image` message.
    /// 
    /// # Example:
    /// ```
    /// let mut cv_image = CvImage::from_imgmsg(image_msg).unwrap();
    /// let image_msg = cv_image.to_imgmsg(0).unwrap();
    /// ```
    /// 
    /// ## Arguments
    /// * `is_bigendian` - Endianness of the image data. 0 for little-endian, 1 for big-endian.
    /// 
    /// ## Returns
    /// * `sensor_msgs::Image` message
    pub fn into_imgmsg(self, is_bigendian: u8) -> Result<Image, Box<dyn Error>> {
        let step = self.width as u32 * image_encodings::get_num_channels(&self.encoding) as u32;
        
        let data = match self.data {
            DataContainer::VecU8(data) => data,
            DataContainer::VecU16(data) => image_byteorder_ops::from_u16_to_u8(&data, is_bigendian == 1),
            _ => Err(format!("Unsupported container type"))?
        };

        Ok(Image {
            header: self.header,
            height: self.height as u32,
            width: self.width as u32,
            encoding: self.encoding,
            is_bigendian: is_bigendian,
            step: step,
            data: data
        })
    }

    /// Converts the `CvImage` to a `CvImage` with a different encoding. It will copy the data
    /// into the new buffer.
    /// 
    /// # Example:
    /// ```
    /// let mut cv_image = CvImage::from_imgmsg(image_msg).unwrap();
    /// let cv_image = cv_image.to_cvimage("mono8").unwrap();
    /// ```
    /// 
    /// ## Arguments
    /// * `desired_encoding` - Encoding of the new image. Check the supported encodings in the
    ///                       `image_encodings` module.
    /// 
    /// ## Returns
    /// * `CvImage` object
    pub fn to_cvimage(&mut self, desired_encoding: &str) -> Result<CvImage, Box<dyn Error>> {
        let src_enc = image_encodings::from_encstr_to_cvenc(&self.encoding)?;
        let dst_enc = image_encodings::from_encstr_to_cvenc(desired_encoding)?;
        let convertion_code = image_encodings::get_conversion_code(src_enc, dst_enc)?;

        let src_mat = self.as_cvmat()?;
        let mut dst_mat = Mat::default();

        opencv::imgproc::cvt_color(&src_mat, &mut dst_mat, convertion_code, 0)?;

        let cvtype = image_encodings::from_encstr_to_cvtype(desired_encoding)?;
        let scaling = image_encodings::get_scaling_factor(&self.encoding, desired_encoding);

        let mut dst2_mat = Mat::default();
        
        dst_mat.convert_to(&mut dst2_mat, cvtype, scaling, 0.0)?;

        Ok(CvImage::from_cvmat(dst2_mat, desired_encoding)?)
    }

    /// Returns the image as a `cv::Mat` object. This is a cheap operation 
    /// as the data is shared between the `CvImage` and the `Mat` object.
    /// 
    /// ## Returns
    /// * `opencv::core::Mat` object
    pub fn as_cvmat(&mut self) -> Result<Mat, Box<dyn Error>> {
        let cvtype = image_encodings::from_encstr_to_cvtype(&self.encoding)?;
        
        let buffer_mut_ptr = match self.data {
            DataContainer::VecU8(ref mut data) => data.as_mut_ptr() as *mut _,
            DataContainer::VecU16(ref mut data) => data.as_mut_ptr() as *mut _,
            _ => Err(format!("Unsupported container type"))?
        };

        let mat;
        unsafe {
            mat = Mat::new_rows_cols_with_data(
                self.height as i32,
                self.width as i32,
                cvtype,
                buffer_mut_ptr,
                opencv::core::Mat_AUTO_STEP
            )?;
        }

        Ok(mat)
    }

    /// Returns the immutable internal vector container containing the image data.
    /// 
    /// ## Returns
    /// * `DataContainer` object
    pub fn as_container(&self) -> &DataContainer {
        &self.data
    }

    /// Returns the mutable internal vector container containing the image data.
    /// 
    /// ## Returns
    /// * `DataContainer` object
    pub fn as_mut_container(&mut self) -> &mut DataContainer {
        &mut self.data
    }

    /// Returns the immutable header message used by ROS.
    /// 
    /// ## Returns
    /// * `std_msgs::Header` message
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns the mutable header message used by ROS.
    /// 
    /// ## Returns
    /// * `std_msgs::Header` message
    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    /// Returns the immutable encoding string. Check the supported encodings in the
    /// `image_encodings` module.
    /// 
    /// ## Returns
    /// * `String` object
    pub fn encoding(&self) -> &String {
        &self.encoding
    }

    /// Returns the immutable width of the image.
    /// 
    /// ## Returns
    /// * `usize` object containing the width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the immutable height of the image.
    /// 
    /// ## Returns
    /// * `usize` object containing the height
    pub fn height(&self) -> usize {
        self.height
    }
}