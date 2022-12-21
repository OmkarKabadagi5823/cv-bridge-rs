use std::io::Cursor;
use byteorder::{
    LittleEndian,
    BigEndian,
    ReadBytesExt,
    WriteBytesExt,
};

pub fn from_u8_to_u16(data: &[u8], big_endian: bool) -> Vec<u16> {
    let mut data16 = Vec::with_capacity(data.len());
    let mut cursor = Cursor::new(data);

    if big_endian {
        while let Ok(value) = cursor.read_u16::<BigEndian>() {
            data16.push(value);
        }
    } else {
        while let Ok(value) = cursor.read_u16::<LittleEndian>() {
            data16.push(value);
        }
    }

    data16
}

pub fn from_u16_to_u8(data: &[u16], big_endian: bool) -> Vec<u8> {
    let mut data8 = Vec::with_capacity(data.len());

    if big_endian {
        for value in data {
            data8.write_u16::<BigEndian>(*value).unwrap();
        }
    } else {
        for value in data {
            data8.write_u16::<LittleEndian>(*value).unwrap();
        }
    }

    data8
}