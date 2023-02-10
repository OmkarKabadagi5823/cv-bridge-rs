//! `cv-bridge` is a crate for converting between OpenCV and ROS Image.
//! This works in conjunction to the [`ros_rust`](https://github.com/adnanademovic/rosrust) crate.
//! 
//! # Crate Status
//! - Currently only supports the standard CV encodings
//! - Currently only supports CV_8U and CV_16U channel depths
//! - Does not support compressed images
//! 
//! # Examples
//! 
//! ## Convert from ROS Image to OpenCV Mat
//! ```
//! use opencv::{
//!     prelude::*,
//!     highgui,
//! };
//! use rosrust_msg::{
//!     sensor_msgs::Image,
//!     std_msgs::Header
//! };
//! use cv_bridge::CvImage;
//! 
//! fn main() {
//!     rosrust::init("image_listener");
//! 
//!     let _subscriber_raii = rosrust::subscribe(
//!         "/camera/image_raw", 
//!         1, 
//!         move |ros_image: Image| {
//!             let mut cv_image = CvImage::from_imgmsg(ros_image).unwrap();
//!             let mat = cv_image.as_cvmat().unwrap();
//! 
//!             highgui::imshow("image", &mat).unwrap();
//!             highgui::wait_key(1).unwrap();
//!         }
//!     ).unwrap();
//! 
//!     rosrust::spin();
//! }
//! ```

pub mod cv_image;
pub mod utils;

pub use cv_image::CvImage;
