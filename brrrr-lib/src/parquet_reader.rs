// (c) Copyright 2022 Trent Hauck
// All Rights Reserved

use noodles::fasta;
use noodles::fastq;
use parquet::file::reader::SerializedFileReader;
use parquet::record::RowAccessor;
use std::io::{self, Result};
use std::{fs::File, path::Path};

/// pq2fa reads an input parquet file, and converts the "id", "sequence", and "description" columns
/// into a FASTA file with the format: ">{id} {description}\n{sequence}".
///
/// # Arguments
///
/// * `input` - The path to the input Parquet file.
/// * `output` - The path to the output FASTA file.
pub fn pq2fa<P: AsRef<Path>>(input: P, output: P) -> Result<()> {
    let output_file = File::create(output).unwrap();
    let handle = io::BufWriter::new(output_file);
    let mut writer = fasta::Writer::new(handle);

    if let Ok(file) = File::open(&input) {
        let reader = SerializedFileReader::new(file).unwrap();

        for row in reader.into_iter() {
            let mut id = None;
            let mut description = None;
            let mut sequence = None;

            for (e, (key, _)) in row.get_column_iter().enumerate() {
                match key.as_str() {
                    "id" => id = Some(row.get_string(e).expect("unable to read id column")),
                    "sequence" => {
                        sequence = Some(row.get_string(e).expect("uanble to read sequence column"))
                    }
                    "description" => {
                        description = match row.get_string(e) {
                            Ok(v) => Some(v.to_string()),
                            Err(_) => None,
                        };
                    }
                    _ => continue,
                }
            }

            match (id, description, sequence) {
                (Some(i), d, Some(s)) => {
                    let definition = fasta::record::Definition::new(i, d);

                    let sequence = fasta::record::Sequence::from(s.as_bytes().to_vec());
                    let record = fasta::Record::new(definition, sequence);
                    writer.write_record(&record)?
                }
                (_, _, _) => {
                    panic!("unable to handle values passed in id, description, or sequence")
                }
            };
        }
    }

    Ok(())
}

/// pq2fq reads an input parquet file and converts it to fastq.
///
/// # Arguments
///
/// * `input` - The path to the input Parquet file.
/// * `output` - The path to the output FASTQ file.
pub fn pq2fq<P: AsRef<Path>>(input: P, output: P) -> Result<()> {
    let output_file = File::create(output).unwrap();
    let handle = io::BufWriter::new(output_file);
    let mut writer = fastq::Writer::new(handle);

    if let Ok(file) = File::open(&input) {
        let reader = SerializedFileReader::new(file).unwrap();

        for row in reader.into_iter() {
            let mut id = None;
            let mut description = None;
            let mut sequence = None;
            let mut quality = None;

            for (e, (key, _)) in row.get_column_iter().enumerate() {
                match key.as_str() {
                    "id" => id = Some(row.get_string(e).expect("unable to read id column")),
                    "sequence" => {
                        sequence = Some(row.get_string(e).expect("unable to read sequence column"))
                    }
                    "quality" => {
                        quality = Some(row.get_string(e).expect("unable to read quality column"))
                    }
                    "description" => {
                        description = match row.get_string(e) {
                            Ok(v) => Some(v.to_string()),
                            Err(_) => None,
                        };
                    }
                    _ => continue,
                }
            }

            match (id, description, sequence, quality) {
                (Some(i), _, Some(s), Some(q)) => {
                    let record = fastq::Record::new(i.as_bytes(), s.as_bytes(), q.as_bytes());
                    writer.write_record(&record)?;
                }
                (_, _, _, _) => {
                    panic!(
                        "unable to handle values passed in id, description, sequence, or quality"
                    )
                }
            };
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, io::BufReader};

    use itertools::Itertools;
    use noodles::fasta::{self, record::Definition, record::Sequence};
    use noodles::fastq;
    use parquet::basic::Compression;

    use super::*;
    use crate::parquet_writer::{fa2pq, fq2pq, BioFileCompression};

    #[test]
    fn parquet_fastq_base_test() {
        let temp_dir = env::temp_dir();
        let initital_fasta = temp_dir.join("initital_fasta.fastq");
        let initial_parquet = temp_dir.join("initial_fq_parquet.parquet");
        let second_fasta = temp_dir.join("second_fasta.fastq");

        let r = fastq::Record::new("r0", "AGCT", "NDLS");

        let mut writer = fastq::Writer::new(File::create(&initital_fasta).expect("error"));
        writer.write_record(&r).expect("error");

        fq2pq(&initital_fasta, &initial_parquet, Compression::UNCOMPRESSED).expect("fa2pq failed");
        assert!(&initial_parquet.exists());
        pq2fq(&initial_parquet, &second_fasta).expect("fa2pq failed");

        let mut reader =
            fastq::Reader::new(BufReader::new(File::open(&second_fasta).expect("error")));

        let recs = reader.records().collect_vec();
        assert_eq!(recs.len(), 1);

        let actual_record = recs.get(0);
        if let Some(ar) = actual_record {
            assert!(ar.is_ok());

            match ar {
                Ok(a) => assert_eq!(a.name(), r.name()),
                Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidData),
            };
        }
    }

    #[test]
    fn parquet_fasta_base_test() {
        let temp_dir = env::temp_dir();
        let initital_fasta = temp_dir.join("initital_fasta.fasta");
        let initial_parquet = temp_dir.join("initial_parquet.parquet");
        let second_fasta = temp_dir.join("second_fasta.fasta");

        let r = fasta::Record::new(
            Definition::new("name", Some("description".to_string())),
            Sequence::from(b"ATCG".to_vec()),
        );

        let mut writer = fasta::Writer::new(File::create(&initital_fasta).expect("error"));
        writer.write_record(&r).expect("error");

        fa2pq(
            &initital_fasta,
            &initial_parquet,
            Compression::UNCOMPRESSED,
            BioFileCompression::UNCOMPRESSED,
        )
        .expect("fa2pq failed");

        assert!(&initial_parquet.exists());
        pq2fa(&initial_parquet, &second_fasta).expect("fa2pq failed");

        let mut reader =
            fasta::Reader::new(BufReader::new(File::open(&second_fasta).expect("error")));

        let recs = reader.records().collect_vec();
        assert_eq!(recs.len(), 1);

        let actual_record = recs.get(0);
        if let Some(ar) = actual_record {
            assert!(ar.is_ok());

            match ar {
                Ok(a) => assert_eq!(a.definition(), r.definition()),
                Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidData),
            };
        }
    }
}
