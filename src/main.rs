// use cv_bridge::CvImage;
// use cv_bridge::utils;
// fn main() {
//     let v: Vec<u16> = vec![65535,32768,255];
//     let u = utils::image_encoding_ops::from_u16_to_u8(&v, false);

//     println!("{:?}", u);
// }

use byteorder::{ByteOrder, LittleEndian, BigEndian};

fn main() {
    let arr: [u8; 32] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,                         0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,                         0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,                         0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20];
    let swapped_endian = arr.chunks(2)
        .map(|chunk| BigEndian::read_u16(chunk))
        .map(|num| num.swap_bytes())
        .flat_map(|num| num.to_ne_bytes().to_vec())
        .collect::<Vec<u8>>();
    println!("{:?}", swapped_endian);
}