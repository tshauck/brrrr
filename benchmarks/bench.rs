// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

extern crate test;

use test::Bencher;

static BENCH_SIZE: usize = 20;

#[bench]
fn jsonl_fasta_write_benchmark(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    b.iter(|| (0..BENCH_SIZE).map(fibonacci).collect::<Vec<u32>>())

}
