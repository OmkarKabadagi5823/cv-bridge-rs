use opencv::highgui;
use cv_bridge::{
    CvImage,
    msgs::sensor_msgs::Image,
};

fn main() {
    // Initialize ros node
    rosrust::init("image_viewer");

    // Create image subscriber
    let _subscriber_raii = rosrust::subscribe(
        "/camera/image_raw",
        5,
        move |image: Image| {
            // Convert ros Image to opencv Mat
            let mut cv_image = CvImage::from_imgmsg(image).expect("failed to construct CvImage from ros Image"); 
            let mat = cv_image.as_cvmat().expect("failed to convert CvImage to Mat");

            // Display image
            let window = "view";
            highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();
            highgui::imshow(window, &mat).unwrap();
            highgui::wait_key(1).unwrap();
        }
    );

    rosrust::spin();
}