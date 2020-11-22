// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs;
use std::io::Result;
use std::str;
use std::sync::Arc;

use bio::io::fasta;
use bio::io::fastq;

use itertools::Itertools;

use arrow::array::*;
use arrow::datatypes::DataType;
use arrow::datatypes::{Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;

/// Converts a FASTA file to Parquet.
///
/// # Arguments
/// * `input` The string representing the path to the input fasta file.
/// * `output` The string representing the path to the output parquet file.
pub fn fa2pq(input: &str, output: &str) -> Result<()> {
    let file_schema = Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("sequence", DataType::Utf8, false),
    ]);

    let input_file = fs::File::open(input).expect("Error opening file.");
    let reader = fasta::Reader::new(input_file);

    let records = reader.records();

    let file = fs::File::create(output).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(file_schema.clone()), None).unwrap();

    let chunk_size = 2usize.pow(20);
    for chunk in records.into_iter().chunks(chunk_size).into_iter() {
        let mut id_builder = StringBuilder::new(2048);
        let mut seq_builder = StringBuilder::new(2048);

        for chunk_i in chunk {
            let record = match chunk_i {
                Ok(r) => r,
                Err(error) => panic!(error),
            };

            id_builder
                .append_value(record.id())
                .expect("Couldn't append id.");

            let sequence = str::from_utf8(record.seq()).unwrap();
            seq_builder
                .append_value(sequence)
                .expect("Couldn't add sequence.");
        }

        let id_array = id_builder.finish();
        let seq_array = seq_builder.finish();

        let rb = RecordBatch::try_new(
            Arc::new(file_schema.clone()),
            vec![Arc::new(id_array), Arc::new(seq_array)],
        )
        .unwrap();

        writer.write(&rb).expect("Couldn't write record batch.");
    }

    writer.close().expect("Couldn't close file.");
    Ok(())
}

/// Converts a FASTQ file to Parquet.
///
/// # Arguments
/// * `input` The string representing the path to the input fasta file.
/// * `output` The string representing the path to the output parquet file.
pub fn fq2pq(input: &str, output: &str) -> Result<()> {
    let file_schema = Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("sequence", DataType::Utf8, false),
        Field::new("quality", DataType::Utf8, false),
    ]);

    let input_file = fs::File::open(input).expect("Error opening file.");
    let reader = fastq::Reader::new(input_file);

    let records = reader.records();

    let file = fs::File::create(output).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(file_schema.clone()), None).unwrap();

    let chunk_size = 2usize.pow(20);
    for chunk in records.into_iter().chunks(chunk_size).into_iter() {
        let mut id_builder = StringBuilder::new(2048);
        let mut seq_builder = StringBuilder::new(2048);
        let mut quality_builder = StringBuilder::new(2048);

        for chunk_i in chunk {
            let record = match chunk_i {
                Ok(r) => r,
                Err(error) => panic!(error),
            };

            id_builder
                .append_value(record.id())
                .expect("Couldn't append id.");

            let sequence = str::from_utf8(record.seq()).unwrap();
            seq_builder
                .append_value(sequence)
                .expect("Couldn't add sequence.");

            let quality = str::from_utf8(record.qual()).unwrap();
            quality_builder
                .append_value(quality)
                .expect("Couldn't add sequence.");
        }

        let id_array = id_builder.finish();
        let seq_array = seq_builder.finish();
        let quality_array = quality_builder.finish();

        let rb = RecordBatch::try_new(
            Arc::new(file_schema.clone()),
            vec![Arc::new(id_array), Arc::new(seq_array), Arc::new(quality_array)],
        )
        .unwrap();

        writer.write(&rb).expect("Couldn't write record batch.");
    }

    writer.close().expect("Couldn't close file.");
    Ok(())
}
