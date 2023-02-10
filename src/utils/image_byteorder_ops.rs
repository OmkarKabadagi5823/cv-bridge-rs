//! This module contains functions to convert between big and little endian byte order.
use std::io::Cursor;
use byteorder::{
    LittleEndian,
    BigEndian,
    ReadBytesExt,
    WriteBytesExt
};

/// Takes a u8 array and constructs a u16 array by converting
/// two u8 values into one u16 value.
/// 
/// ## Arguments
/// * `data` - The u8 array to convert.
/// * `big_endian` - If true, the u8 array is in big endian byte order.
///                 If false, the u8 array is in little endian byte order.
/// 
/// ## Returns
/// A u16 array.
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

/// Takes a u16 array and constructs a u8 array by converting
/// one u16 value into two u8 values.
/// 
/// ## Arguments
/// * `data` - The u16 array to convert.
/// * `big_endian` - If true, the u8 array will be in big endian byte order.
///                If false, the u8 array will be in little endian byte order.
/// 
/// ## Returns
/// A u8 array.
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

/// Takes a u8 array in big endian byte order and converts it to little endian byte order.
/// 
/// ## Arguments
/// * `be` - The u8 array to convert.
/// 
/// ## Returns
/// A u8 array in little endian byte order.
pub fn from_be_to_le(be: &[u8]) -> Vec<u8> {
    let mut le = Vec::with_capacity(be.len());
    let mut cursor = Cursor::new(be);

    while let Ok(value) = cursor.read_u16::<BigEndian>() {
        le.write_u16::<LittleEndian>(value).unwrap();
    }

    le
}

/// Takes a u8 array in little endian byte order and converts it to big endian byte order.
/// 
/// ## Arguments
/// * `le` - The u8 array to convert.
/// 
/// ## Returns
/// A u8 array in big endian byte order.
pub fn from_le_to_be(le: &[u8]) -> Vec<u8> {
    let mut be = Vec::with_capacity(le.len());
    let mut cursor = Cursor::new(le);

    while let Ok(value) = cursor.read_u16::<LittleEndian>() {
        be.write_u16::<BigEndian>(value).unwrap();
    }

    be
}
