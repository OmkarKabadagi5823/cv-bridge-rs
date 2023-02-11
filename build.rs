use std::{
    env,
    path::PathBuf,
};

fn main() {
    let ros_msg_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR"))
        .join("ros_msgs");
    println!("cargo:rustc-env=ROSRUST_MSG_PATH={}", ros_msg_path.to_str().expect("Failed to set ROSRUST_MSG_PATH"));
}
