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
``` rust
use opencv::{
    prelude::*,
    highgui,
};
use rosrust_msg::{
    sensor_msgs::Image,
    std_msgs::Header
};
use cv_bridge::CvImage;

fn main() {
    // Initialize ROS node
    rosrust::init("image_listener");

    // Create a subscription to the /camera/image_raw topic
    let _subscriber_raii = rosrust::subscribe(
        "/camera/image_raw", 
        1, 
        move |ros_image: Image| {
            // Create a CvImage from the ROS image message
            let mut cv_image = CvImage::from_imgmsg(ros_image).unwrap();

            // Create opencv::core::Mat from the CvImage
            let mat = cv_image.as_cvmat().unwrap();

            // Display the image
            highgui::imshow("image", &mat).unwrap();
            highgui::wait_key(1).unwrap();
        }
    ).unwrap();

    rosrust::spin();
}
```

## Features
- [x] Covert to and from sensor_msgs/Image and opencv::core::Mat
- [x] Support for various encodings defined by [sensor_msgs: image_encodings.h](http://docs.ros.org/en/jade/api/sensor_msgs/html/image__encodings_8h_source.html) crate
- [x] Support for 8-bit and 16-bit depth channels
- [ ] Support for 32-bit and 64-bit depth channels
- [x] Documentation and examples
- [ ] Covert to and from sensor_msgs/CompressedImage and opencv::core::Mat