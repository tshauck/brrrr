// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

#![feature(test)]

use criterion::{criterion_group, criterion_main, Criterion};

use std::fs::File;
use bio::io::fasta;

extern crate brrrr;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Write 1000 records.", |b| {
        let mut tmpfile: File = tempfile::tempfile().unwrap();

        b.iter(|| {
            let _ = nom_pdb::Parser::parse(PDB_7ZNF);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
