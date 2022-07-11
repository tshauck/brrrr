// (c) Copyright 2022 Trent Hauck
// All Rights Reserved

use noodles::fasta;
use noodles::fastq;
use parquet::data_type::AsBytes;
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
                    "id" => id = Some(row.get_bytes(e).expect("unable to read id column")),
                    "sequence" => {
                        sequence = Some(row.get_bytes(e).expect("uanble to read sequence column"))
                    }
                    "description" => {
                        description =
                            Some(row.get_bytes(e).expect("unable to read description column"))
                    }
                    _ => continue,
                }
            }

            match (id, description, sequence) {
                (Some(i), _, Some(s)) => {
                    let definition =
                        fasta::record::Definition::new(i.as_utf8().expect("error"), None);

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
                    "id" => id = Some(row.get_bytes(e).expect("unable to read id column")),
                    "sequence" => {
                        sequence = Some(row.get_bytes(e).expect("unable to read sequence column"))
                    }
                    "quality" => {
                        quality = Some(row.get_bytes(e).expect("unable to read quality column"))
                    }
                    "description" => {
                        description = Some(
                            row.get_string(e)
                                .expect("unable to read description column"),
                        )
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
