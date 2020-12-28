// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

#![feature(test)]

use criterion::{criterion_group, criterion_main, Criterion};

use std::env;
use std::fs::File;
use std::io::sink;

extern crate brrrr_lib;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Write 10000 records.", |b| {
        let path: &'static str = env!("BENCH_DATA");

        b.iter(|| {
            let filename = format!("./{}/10000.fasta", path);
            let f = File::open(filename).expect("Error opening file.");
            let _ = brrrr_lib::json_writer::fa2jsonl(f, &mut sink());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
