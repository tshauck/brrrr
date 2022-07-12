// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use criterion::{criterion_group, criterion_main, Criterion};

use parquet::basic::Compression;
use std::env;
use std::fs::File;
use std::io::{sink, BufReader};
use std::time::Duration;

extern crate brrrr_lib;

fn criterion_benchmark(c: &mut Criterion) {
    let path: &'static str = env!("BENCH_DATA");

    c.bench_function("Write 10000 records from FASTA to jsonl.", |b| {
        b.iter(|| {
            let filename = format!("./{}/10000.fasta", path);
            let f = File::open(filename).expect("Error opening file.");
            let _ = brrrr_lib::json_writer::fa2jsonl(BufReader::new(f), &mut sink());
        })
    });
}

fn bench_fasta_parquet_output(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_fasta_parquet_output");
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(30));

    let path: &'static str = env!("BENCH_DATA");
    let temp_path = env::temp_dir();

    let test_cases = vec![
        (
            Compression::UNCOMPRESSED,
            brrrr_lib::parquet_writer::BioFileCompression::UNCOMPRESSED,
            "10000.fasta",
        ),
        (
            Compression::UNCOMPRESSED,
            brrrr_lib::parquet_writer::BioFileCompression::GZIP,
            "10000.fasta.gz",
        ),
    ];

    for (pq_compression, f_compression, fname) in test_cases {
        group.bench_function(
            format!(
                "Write FA to PQ, case {:?} {:?} {}",
                pq_compression, f_compression, fname
            )
            .as_str(),
            |b| {
                b.iter(|| {
                    let filename = format!("./{}/{}", path, fname);

                    let out_file = format!(
                        "{}/{:?}-{:?}-{}.parquet",
                        temp_path.display(),
                        pq_compression,
                        f_compression,
                        fname
                    );

                    let _ = brrrr_lib::parquet_writer::fa2pq(
                        &filename,
                        &out_file,
                        pq_compression,
                        f_compression,
                    );
                })
            },
        );
    }
}

criterion_group!(benches, bench_fasta_parquet_output, criterion_benchmark);
criterion_main!(benches);
