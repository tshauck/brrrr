// (c) Copyright 2021 Trent Hauck
// All Rights Reserved

use quick_xml::de::{from_reader, DeError};

use std::io::BufRead;
use std::result::Result;

use crate::types;
use crate::types::DecodedArray;

pub fn parse_mzml<R: BufRead>(reader: R) -> Result<types::MzML, DeError> {
    let mzml: types::MzML = from_reader(reader)?;
    Ok(mzml)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn parse_mzml_test() {
        let filename = "./data/test.xml";

        let file = File::open(filename).expect("Couldn't open");
        let buf_reader = BufReader::new(file);

        let mzml = parse_mzml(buf_reader).expect("Couldn't parse MzML");

        assert_eq!(mzml.run.id, "Exp01-PDA");

        assert_eq!(mzml.run.spectrum_list.spectrum.len(), 1);
        assert_eq!(mzml.run.chromatogram_list.chromatogram.len(), 1);

        let spectrum = &mzml.run.spectrum_list.spectrum[0];
        let data_array = spectrum.decode_array(0).unwrap();

        assert_eq!(
            data_array,
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0]
        );
    }
}
