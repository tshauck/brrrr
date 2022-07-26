// (c) Copyright 2022 Trent Hauck
// All Rights Reserved

use noodles::core;
use noodles::fasta;
use noodles::fastq;
use noodles::gff;
use noodles::gff::record::attributes::Entry;
use noodles::gff::record::Attributes;
use noodles::gff::record::Phase;
use noodles::gff::record::Strand;
use parquet::file::reader::SerializedFileReader;
use parquet::record::RowAccessor;
use std::io;
use std::{fs::File, path::Path};

use crate::errors::BrrrrError;

/// pq2fa reads an input parquet file, and converts the "id", "sequence", and "description" columns
/// into a FASTA file with the format: ">{id} {description}\n{sequence}".
///
/// # Arguments
///
/// * `input` - The path to the input Parquet file.
/// * `output` - The path to the output FASTA file.
pub fn pq2fa<P: AsRef<Path>>(input: P, output: P) -> Result<(), BrrrrError> {
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
                    "id" => id = Some(row.get_string(e)?),
                    "sequence" => sequence = Some(row.get_string(e)?),
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
                    return Err(BrrrrError::IOError(io::Error::new(
                        io::ErrorKind::Unsupported,
                        format!(
                            "Unexpected parsing for id: {}",
                            id.unwrap_or(&String::from("unknown id")),
                        ),
                    )))
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
pub fn pq2fq<P: AsRef<Path>>(input: P, output: P) -> Result<(), BrrrrError> {
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
                    "id" => id = Some(row.get_string(e)?),
                    "sequence" => sequence = Some(row.get_string(e)?),
                    "quality" => quality = Some(row.get_string(e)?),
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

/// pq2gff reads an input parquet file and convers it to GFF.
///
/// # Arguments
///
/// * `input` - The path to the input Parquet file.
/// * `output` - The path to the output GFF file.

pub fn pq2gff<P: AsRef<Path>>(input: P, output: P) -> Result<(), BrrrrError> {
    let output_file = File::create(output).unwrap();
    let handle = io::BufWriter::new(output_file);
    let mut writer = gff::Writer::new(handle);

    if let Ok(file) = File::open(&input) {
        let reader = SerializedFileReader::new(file).unwrap();

        for row in reader.into_iter() {
            let mut gff_record_builder = gff::Record::builder();

            for (e, (key, _)) in row.get_column_iter().enumerate() {
                match key.as_str() {
                    "seqname" => {
                        gff_record_builder = gff_record_builder
                            .set_reference_sequence_name(row.get_string(e)?.to_string())
                    }
                    "source" => {
                        gff_record_builder =
                            gff_record_builder.set_source(row.get_string(e)?.to_string())
                    }
                    "feature_type" => {
                        gff_record_builder =
                            gff_record_builder.set_type(row.get_string(e)?.to_string())
                    }
                    "start" => {
                        let int_position = row.get_long(e)?;
                        let position = core::Position::new(int_position as usize).unwrap();
                        gff_record_builder = gff_record_builder.set_start(position);
                    }
                    "end" => {
                        let int_position = row.get_long(e)?;
                        let position = core::Position::new(int_position as usize).unwrap();
                        gff_record_builder = gff_record_builder.set_end(position);
                    }
                    "score" => match row.get_long(e) {
                        Ok(score) => {
                            gff_record_builder = gff_record_builder.set_score(score as f32);
                        }
                        _ => continue,
                    },
                    "stand" => {
                        let strand = row.get_string(e)?;
                        let n_strand = strand.parse::<Strand>().expect("unable to parse strand");
                        gff_record_builder = gff_record_builder.set_strand(n_strand);
                    }
                    "frame" => {
                        let frame = row.get_string(e);
                        match frame {
                            Ok(f) => {
                                let phase = f.parse::<Phase>().expect("unable to parse phase");
                                gff_record_builder = gff_record_builder.set_phase(phase);
                            }
                            _ => continue,
                        }
                    }
                    "attributes" => {
                        let parquet_map = row.get_map(e)?;
                        let entries: Vec<Entry> = parquet_map
                            .entries()
                            .iter()
                            .map(|entry| {
                                let (key, value) = &entry;

                                // TODO(trent): why is does this need trim_matches?
                                Entry::new(
                                    key.to_string().trim_matches('"'),
                                    value.to_string().trim_matches('"'),
                                )
                            })
                            .collect();

                        let attributes = Attributes::from(entries);
                        gff_record_builder = gff_record_builder.set_attributes(attributes);
                    }
                    _ => continue,
                }
            }

            writer.write_record(&gff_record_builder.build())?;
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
    use crate::parquet_writer::{fa2pq, fq2pq, gff2pq, BioFileCompression};

    #[test]
    fn parquet_gff_base_test() {
        let temp_dir = env::temp_dir();
        let initital_gff = temp_dir.join("initital_gff.gff");
        let initial_parquet = temp_dir.join("initial_gff_parquet.parquet");
        let second_gff = temp_dir.join("second_gff.gff");

        let s = "sq0\tNOODLES\tgene\t8\t13\t.\t+\t.\tgene_id=ndls0;gene_name=gene0";
        let gff_record = s.parse::<gff::Record>().expect("parse error");

        let mut writer = gff::Writer::new(File::create(&initital_gff).expect("error"));
        writer.write_record(&gff_record).expect("error");

        gff2pq(&initital_gff, &initial_parquet, Compression::UNCOMPRESSED).expect("gff2pq failed");
        assert!(&initial_parquet.exists());
        pq2gff(&initial_parquet, &second_gff).expect("pq2gff failed");

        let mut reader = gff::Reader::new(BufReader::new(File::open(&second_gff).expect("error")));
        let recs = reader.records().collect_vec();
        assert_eq!(recs.len(), 1);

        let actual_record = recs.get(0);
        if let Some(ar) = actual_record {
            assert!(ar.is_ok());

            match ar {
                Ok(found_gff_record) => {
                    assert_eq!(
                        found_gff_record.reference_sequence_name(),
                        gff_record.reference_sequence_name()
                    );

                    let mut found_keys: Vec<&str> = found_gff_record
                        .attributes()
                        .into_iter()
                        .map(|e| e.key())
                        .collect();
                    let mut gff_keys: Vec<&str> = gff_record
                        .attributes()
                        .into_iter()
                        .map(|e| e.key())
                        .collect();
                    assert_eq!(found_keys.sort(), gff_keys.sort());

                    assert_eq!(found_gff_record.source(), gff_record.source());
                    assert_eq!(found_gff_record.ty(), gff_record.ty());
                    assert_eq!(found_gff_record.start(), gff_record.start());
                    assert_eq!(found_gff_record.end(), gff_record.end());
                    assert_eq!(found_gff_record.score(), gff_record.score());
                    assert_eq!(found_gff_record.phase(), gff_record.phase());
                }
                _ => panic!("could not match gff"),
            }
        }
    }

    #[test]
    fn parquet_fastq_base_test() {
        let temp_dir = env::temp_dir();
        let initital_fasta = temp_dir.join("initital_fasta.fastq");
        let initial_parquet = temp_dir.join("initial_fq_parquet.parquet");
        let second_fasta = temp_dir.join("second_fastq.fastq");

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
