// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use serde::{Deserialize, Serialize};

use byteorder::{LittleEndian, ReadBytesExt};

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io::Cursor;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CVParam {
    pub cv_ref: String,
    pub accession: String,
    pub name: String,
    pub value: Option<String>,
    pub unit_accession: Option<String>,
    pub unit_name: Option<String>,
    pub unit_cv_ref: Option<String>,
}

impl CVParam {
    pub fn get_data_type(&self) -> Result<DataType, MissingDataTypeError> {
        let dt = DataType::try_from(self)?;
        Ok(dt)
    }

    pub fn new(
        cv_ref: String,
        accession: String,
        name: String,
        value: Option<String>,
        unit_accession: Option<String>,
        unit_name: Option<String>,
        unit_cv_ref: Option<String>,
    ) -> Self {
        Self {
            cv_ref,
            accession,
            name,
            value,
            unit_accession,
            unit_name,
            unit_cv_ref,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserParam {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScanWindow {
    pub cv_param: CVVector,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScanWindowList {
    pub scan_window: Vec<ScanWindow>,
    pub count: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Scan {
    pub cv_param: CVVector,
    pub scan_window_list: ScanWindowList,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScanList {
    pub cv_param: CVVector,
    pub scans: Vec<Scan>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Binary {
    #[serde(rename = "$value")]
    pub content: String,
}

impl Binary {
    pub fn new(content: String) -> Binary {
        Binary { content }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryDataArray {
    pub encoded_length: String,
    pub cv_param: CVVector,
    pub binary: Binary,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum DataType {
    Int64Bit,
    Float64Bit,
}

impl TryFrom<CVVector> for DataType {
    type Error = MissingDataTypeError;

    fn try_from(value: CVVector) -> Result<Self, Self::Error> {
        for cv_param in value.iter() {
            let data_type = DataType::try_from(cv_param);
            if !data_type.is_err() {
                return Ok(data_type.unwrap());
            }
        }
        Err(MissingDataTypeError)
    }
}

#[derive(Debug, Clone)]
pub struct MissingDataTypeError;

impl fmt::Display for MissingDataTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for MissingDataTypeError {}

impl TryFrom<&CVParam> for DataType {
    type Error = MissingDataTypeError;

    fn try_from(value: &CVParam) -> Result<Self, Self::Error> {
        match value.accession.as_str() {
            "MS:1000522" => Ok(DataType::Float64Bit),
            _ => Err(MissingDataTypeError),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CompressionType {
    NoCompression,
    ZlibCompression,
}

#[derive(Debug, Clone)]
pub struct MissingCompressionError;

impl fmt::Display for MissingCompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "missing compression in controlled vocabulary parameters")
    }
}

impl Error for MissingCompressionError {}

type CVVector = Vec<CVParam>;

trait CVVectorMethods {}

impl CVVectorMethods for CVVector {}

impl TryFrom<CVVector> for CompressionType {
    type Error = MissingCompressionError;

    fn try_from(value: CVVector) -> Result<Self, Self::Error> {
        for cv_param in value.iter() {
            let compression_type = CompressionType::try_from(cv_param);
            if !compression_type.is_err() {
                return Ok(compression_type.unwrap());
            }
        }
        Err(MissingCompressionError)
    }
}

impl TryFrom<&CVParam> for CompressionType {
    type Error = MissingDataTypeError;

    fn try_from(value: &CVParam) -> Result<Self, Self::Error> {
        match value.accession.as_str() {
            "MS:1000576" => Ok(CompressionType::NoCompression),
            "MS:1000574" => Ok(CompressionType::ZlibCompression),
            _ => Err(MissingDataTypeError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataArrayDecodingError;

impl fmt::Display for DataArrayDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for DataArrayDecodingError {}
//https://docs.rs/fastobo/0.13.1/fastobo/ast/struct.OboDoc.html
//https://raw.githubusercontent.com/HUPO-PSI/psi-ms-CV/master/psi-ms.obo

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryDataArrayList {
    pub binary_data_array: Vec<BinaryDataArray>,

    pub count: String,
}

type DecodeArrayError = &'static str;
type DecodedArrayResult<T> = Result<T, DecodeArrayError>;

pub trait DecodedArray {
    fn decode_array(&self, i: usize) -> DecodedArrayResult<Vec<f64>> {
        let de = self.decompress_binary_string(i);
        let decoded = base64::decode(de);

        if decoded.is_ok() {
            let v = decoded.unwrap();

            let mut rdr = Cursor::new(v);

            let mut peaks = Vec::<f64>::new();
            while let Ok(fl) = rdr.read_f64::<LittleEndian>() {
                peaks.push(fl);
            }
            return Ok(peaks);
        };
        Err("error")
    }

    fn decompress_binary_string(&self, i: usize) -> &String;
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Spectrum {
    pub cv_param: CVVector,
    pub index: String,
    pub id: String,
    pub default_array_length: String,
    pub binary_data_array_list: BinaryDataArrayList,
}

impl DecodedArray for Spectrum {
    fn decompress_binary_string(&self, i: usize) -> &String {
        &self.binary_data_array_list.binary_data_array[i]
            .binary
            .content
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpectrumList {
    pub spectrum: Vec<Spectrum>,
    pub count: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Chromatogram {
    pub cv_param: CVVector,
    pub index: String,
    pub id: String,
    pub default_array_length: String,
    pub binary_data_array_list: BinaryDataArrayList,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChromatogramList {
    pub chromatogram: Vec<Chromatogram>,
    pub count: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub id: String,
    pub spectrum_list: SpectrumList,
    pub chromatogram_list: ChromatogramList,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingMethod {
    pub software_ref: String,
    pub cv_param: CVVector,
    pub user_param: Vec<UserParam>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataProcessing {
    pub id: String,
    pub processing_method: Vec<ProcessingMethod>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataProcessingList {
    pub data_processing: Vec<DataProcessing>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileContent {
    cv_param: CVVector,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceFile {
    pub id: String,
    pub name: String,
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceFileList {
    source_file: SourceFile,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileDescription {
    pub file_content: FileContent,

    pub source_file_list: SourceFileList,
    pub cv_param: CVVector,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MzML {
    pub run: Run,
    pub data_processing_list: DataProcessingList,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_from_cvvector_test() {
        let cv_param = CVParam::new(
            String::from("test"),
            String::from("MS:1000576"),
            String::from("test"),
            None,
            None,
            None,
            None,
        );

        let ctype = CompressionType::try_from(&cv_param).unwrap();
        assert_eq!(ctype, CompressionType::NoCompression);

        let cv_params = vec![cv_param];

        let new_ctype = CompressionType::try_from(cv_params).unwrap();
        assert_eq!(new_ctype, CompressionType::NoCompression);
    }
}
