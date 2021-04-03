// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use serde::de::{self, Deserializer, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use flate2::read::ZlibDecoder;
use std::io::prelude::*;

use std::fmt;

use crate::types::{Binary, CompressionType, DataType};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub fn decode_binary_array(b: &Binary, dt: &DataType, ct: &CompressionType) -> Vec<f64> {
    let decoded = base64::decode(&b.content).expect("FUCK");

    match ct {
        CompressionType::NoCompression => binary_string_to_array(decoded),
        CompressionType::ZlibCompression => {
            let mut decoded_bytes = Vec::<u8>::new();

            println!("{:?}", b.content.as_bytes());

            let rdr = Cursor::new(decoded);

            let mut d = ZlibDecoder::new(rdr);
            d.read_to_end(&mut decoded_bytes).unwrap();

            binary_string_to_array(decoded_bytes)
        }
    }
}

fn binary_string_to_array(decoded: Vec<u8>) -> Vec<f64> {
    let mut rdr = Cursor::new(decoded);
    let mut peaks = Vec::<f64>::new();

    while let Ok(fl) = rdr.read_f64::<LittleEndian>() {
        peaks.push(fl);
    }

    peaks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mzml_test() {
        struct TestData {
            binary: Binary,
            compression_type: CompressionType,
            data_type: DataType,
            expected_array: Vec<f64>,
        }

        impl TestData {
            pub fn new(
                binary: Binary,
                compression_type: CompressionType,
                data_type: DataType,
                expected_array: Vec<f64>,
            ) -> Self {
                Self {
                    binary,
                    compression_type,
                    data_type,
                    expected_array,
                }
            }
        }

        let tests = vec![
            TestData::new(Binary::new(String::from("AAAAAAAALkAAAAAAAAAsQAAAAAAAACpAAAAAAAAAKEAAAAAAAAAmQAAAAAAAACRAAAAAAAAAIkAAAAAAAAAgQAAAAAAAABxAAAAAAAAAGEAAAAAAAAAUQAAAAAAAABBAAAAAAAAACEAAAAAAAAAAQAAAAAAAAPA/")), CompressionType::NoCompression, DataType::Float64Bit, vec![15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]),
            TestData::new(Binary::new(String::from("eJxjYEABDhBKAEpLQGkFKK0CpTWgtA6UNoDSRg4AZlQDYw==")), CompressionType::ZlibCompression, DataType::Float64Bit, vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0])
        ];

        for test in tests.iter() {
            let array = decode_binary_array(&test.binary, &test.data_type, &test.compression_type);
            assert_eq!(array, test.expected_array);
        }
    }
}
