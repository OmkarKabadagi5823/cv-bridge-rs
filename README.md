# cv-bridge-rs
[![Crates.io](https://img.shields.io/crates/v/cv-bridge.svg)](https://crates.io/crates/cv-bridge)
[![Docs.rs](https://docs.rs/cv-bridge/badge.svg)](https://docs.rs/cv-bridge)
<img src="https://img.shields.io/badge/built_with-Rust-dca282.svg">

Rust implemenation of cv_bridge that converts between ROS image messages and OpenCV images

> **Warning**: This package is still under active development. Use at your own risk.  
## Getting Started
### Adding cv_bridge to your project
Add the following to your Cargo.toml file under dependencies:
```toml
[dependencies]
cv-bridge = "0.2.0"
```
or you can use cargo to add the dependency:
```bash
cargo add cv_bridge
```

### Converting between ROS image messages and OpenCV images
```rust
use cv_bridge::CvImage;
use rosrust::sensor_msgs::Image;
use opencv::prelude::*;

fn main() {
    // Convert from ROS image message to OpenCV image
    let ros_image = Image::default();
    let cv_image = CvImage::from_imgmsg(ros_image);
    let mat = cv_image.as_cvmat("rgb8".to_string());
   
    // Convert from OpenCV image to ROS image message
    let cv_image = CvImage::from_cvmat(mat);
    let ros_image = cv_image.into_imgmsg();
}
```

### Using the underlying DynamicImage type
```rust
use cv_bridge::CvImage;
use rosrust::sensor_msgs::Image;
use image::DynamicImage;

fn main() {
    // Convert from ROS image message to 
    let ros_image = Image::default();
    let cv_image = CvImage::from_imgmsg(ros_image);

    // Using the underlying DynamicImage type
    let image = cv_image.as_image(); // Returns a immutable reference to the underlying DynamicImage
    let image = cv_image.as_mut_image(); // Returns a mutable reference to the underlying DynamicImage
    
}
```

### Creating CvImage from a vector of bytes
```rust
use cv_bridge::CvImage;
use std::vec;

fn main() {
    let bytes:Vec<u8> = vec![0, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 0]; // 3x3 image with 3 channels
    let cv_image = CvImage::from_vec(3, 3, "rgb8".to_string(), bytes); // width, height, encoding, bytes
}
```

## Features
- [x] Covert to and from sensor_msgs/Image and opencv::core::Mat
- [x] Support for various encodings defined in [image](https://docs.rs/image/latest/image/enum.ColorType.html) crate
- [x] Allows working with the internal DynamicImage type of the [image](https://docs.rs/image/latest/image/enum.DynamicImage.html) crate
- [ ] Covert to and from sensor_msgs/CompressedImage and opencv::core::Mat
- [ ] Documentation and examples
- [ ] Tutorial