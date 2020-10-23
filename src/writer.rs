// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use bio::io::fasta;
use bio::io::fastq;
use bio::io::gff;

use std::io::Result;

/// A RecordWriter writes FASTA records to the underlying source.
pub trait RecordWriter {
    fn write_fasta_record(&mut self, f: fasta::Record) -> Result<()>;
    fn write_fastq_record(&mut self, f: fastq::Record) -> Result<()>;
    fn write_gff_record(&mut self, f: gff::Record) -> Result<()>;
}
