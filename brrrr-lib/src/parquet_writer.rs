// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use std::fs;
use std::io::Result;
use std::str;
use std::sync::Arc;

use bio::io::fasta;
use bio::io::fastq;

use bio::io::gff;
use bio::io::gff::GffType;
use itertools::Itertools;

use arrow::array::*;
use arrow::datatypes::*;
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

pub fn gff2pq(input: &str, output: &str) -> Result<()> {
    let file_schema = Schema::new(vec![
        Field::new("seqname", DataType::Utf8, false),
        Field::new("source", DataType::Utf8, true),
        Field::new("feature_type", DataType::Utf8, false),
        Field::new("start", DataType::Int64, false),
        Field::new("end", DataType::Int64, false),
        Field::new("score", DataType::Int64, false),
        Field::new("strand", DataType::Utf8, false),
        Field::new("frame", DataType::Utf8, false),
        Field::new(
            "attributes",
            DataType::Map(
                Box::new(Field::new(
                    "entries",
                    DataType::Struct(vec![
                        Field::new("keys", DataType::Utf8, false),
                        Field::new("values", DataType::Utf8, true),
                    ]),
                    false,
                )),
                false,
            ),
            false,
        ),
    ]);

    let input_file = fs::File::open(input).expect("Error opening file.");
    let mut reader = gff::Reader::new(input_file, GffType::GTF2);

    let records = reader.records();

    let file = fs::File::create(output).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(file_schema.clone()), None).unwrap();
    let chunk_size = 2usize.pow(20);

    for chunk in records.into_iter().chunks(chunk_size).into_iter() {
        let mut seqname_builder = StringBuilder::new(2048);
        let mut source_builder = StringBuilder::new(2048);
        let mut feature_type_builder = StringBuilder::new(2048);
        let mut start_builder = Int64Builder::new(2048);
        let mut end_builder = Int64Builder::new(2048);
        let mut score_builder = Int64Builder::new(2048);
        let mut strand_builder = StringBuilder::new(2048);
        let mut frame_builder = StringBuilder::new(2048);

        let key_builder = StringBuilder::new(20);
        let value_builder = StringBuilder::new(20);
        let mut att_builder = MapBuilder::new(None, key_builder, value_builder);

        for chunk_i in chunk {
            let record = match chunk_i {
                Ok(r) => r,
                Err(error) => panic!("{}", error),
            };

            seqname_builder
                .append_value(record.seqname())
                .expect("Couldn't append seqname_builder.");

            source_builder
                .append_value(record.source())
                .expect("Couldn't append seqname_builder.");

            feature_type_builder
                .append_value(record.feature_type())
                .expect("Couldn't append seqname_builder.");

            start_builder
                .append_value(*record.start() as i64)
                .expect("Couldn't append seqname_builder.");

            end_builder
                .append_value(*record.end() as i64)
                .expect("Couldn't append seqname_builder.");

            match record.score() {
                Some(score) => score_builder
                    .append_value(score as i64)
                    .expect("Couldn't append seqname_builder."),
                None => score_builder.append_null().expect("error"),
            }

            match record.strand() {
                Some(strand) => strand_builder
                    .append_value(strand.to_string())
                    .expect("Couldn't append seqname_builder."),
                None => strand_builder.append_null().expect("error"),
            }

            frame_builder
                .append_value(record.frame())
                .expect("Couldn't append seqname_builder.");

            let record_key_builder = att_builder.keys();
            for k in record.attributes().keys() {
                record_key_builder.append_value(k).unwrap();
            }

            let record_value_builder = att_builder.values();
            for (_, v) in record.attributes().iter() {
                record_value_builder.append_value(v).unwrap();
            }

            att_builder.append(true).unwrap();
        }

        let seqname_array = seqname_builder.finish();
        let source_array = source_builder.finish();
        let feature_type_array = feature_type_builder.finish();
        let start_array = start_builder.finish();
        let end_array = end_builder.finish();
        let score_array = score_builder.finish();
        let strand_array = strand_builder.finish();
        let frame_array = frame_builder.finish();

        let att_array = att_builder.finish();

        let rb = RecordBatch::try_new(
            Arc::new(file_schema.clone()),
            vec![
                Arc::new(seqname_array),
                Arc::new(source_array),
                Arc::new(feature_type_array),
                Arc::new(start_array),
                Arc::new(end_array),
                Arc::new(score_array),
                Arc::new(strand_array),
                Arc::new(frame_array),
                Arc::new(att_array),
            ],
        )
        .unwrap();

        writer.write(&rb).expect("Couldn't write record batch.");
    }

    writer.close().expect("Couldn't close file.");

    Ok(())
}

/// Converts a FASTA file to Parquet.
///
/// # Arguments
/// * `input` The string representing the path to the input fasta file.
/// * `output` The string representing the path to the output parquet file.
/// * `parquet_compression` The parquet compression to use.
pub fn fa2pq(input: &str, output: &str, parquet_compression: Compression) -> Result<()> {
    let file_schema = Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("description", DataType::Utf8, true),
        Field::new("sequence", DataType::Utf8, false),
    ]);

    let props = WriterProperties::builder()
        .set_compression(parquet_compression)
        .set_statistics_enabled(true);

    let input_file = fs::File::open(input).expect("Error opening file.");
    let reader = fasta::Reader::new(input_file);

    let records = reader.records();

    let file = fs::File::create(output).unwrap();
    let mut writer =
        ArrowWriter::try_new(file, Arc::new(file_schema.clone()), Some(props.build())).unwrap();

    let chunk_size = 2usize.pow(20);
    for chunk in records.into_iter().chunks(chunk_size).into_iter() {
        let mut id_builder = StringBuilder::new(2048);
        let mut description_builder = StringBuilder::new(2048);
        let mut seq_builder = StringBuilder::new(2048);

        for chunk_i in chunk {
            let record = match chunk_i {
                Ok(r) => r,
                Err(error) => panic!("{}", error),
            };

            id_builder
                .append_value(record.id())
                .expect("Couldn't append id.");

            match record.desc() {
                Some(x) => description_builder
                    .append_value(x)
                    .expect("Couldn't append description."),
                _ => description_builder
                    .append_null()
                    .expect("Couldn't append null description."),
            }

            let sequence = str::from_utf8(record.seq()).unwrap();
            seq_builder
                .append_value(sequence)
                .expect("Couldn't add sequence.");
        }

        let id_array = id_builder.finish();
        let desc_array = description_builder.finish();
        let seq_array = seq_builder.finish();

        let rb = RecordBatch::try_new(
            Arc::new(file_schema.clone()),
            vec![
                Arc::new(id_array),
                Arc::new(desc_array),
                Arc::new(seq_array),
            ],
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
/// * `parquet_compression` The parquet compression to use.
pub fn fq2pq(input: &str, output: &str, parquet_compression: Compression) -> Result<()> {
    let file_schema = Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("sequence", DataType::Utf8, false),
        Field::new("description", DataType::Utf8, true),
        Field::new("quality", DataType::Utf8, false),
    ]);

    let props = WriterProperties::builder()
        .set_compression(parquet_compression)
        .set_statistics_enabled(true);

    let input_file = fs::File::open(input).expect("Error opening file.");
    let reader = fastq::Reader::new(input_file);

    let records = reader.records();

    let file = fs::File::create(output).unwrap();
    let mut writer =
        ArrowWriter::try_new(file, Arc::new(file_schema.clone()), Some(props.build())).unwrap();

    let chunk_size = 2usize.pow(20);
    for chunk in records.into_iter().chunks(chunk_size).into_iter() {
        let mut id_builder = StringBuilder::new(2048);
        let mut description_builder = StringBuilder::new(2048);
        let mut seq_builder = StringBuilder::new(2048);
        let mut quality_builder = StringBuilder::new(2048);

        for chunk_i in chunk {
            let record = match chunk_i {
                Ok(r) => r,
                Err(error) => panic!("{}", error),
            };

            id_builder
                .append_value(record.id())
                .expect("Couldn't append id.");

            match record.desc() {
                Some(x) => description_builder
                    .append_value(x)
                    .expect("Couldn't append description."),
                _ => description_builder
                    .append_null()
                    .expect("Couldn't append null description."),
            }

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
        let desc_array = description_builder.finish();
        let seq_array = seq_builder.finish();
        let quality_array = quality_builder.finish();

        let rb = RecordBatch::try_new(
            Arc::new(file_schema.clone()),
            vec![
                Arc::new(id_array),
                Arc::new(seq_array),
                Arc::new(desc_array),
                Arc::new(quality_array),
            ],
        )
        .unwrap();

        writer.write(&rb).expect("Couldn't write record batch.");
    }

    writer.close().expect("Couldn't close file.");
    Ok(())
}
