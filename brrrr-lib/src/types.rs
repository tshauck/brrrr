// (c) Copyright 2022 Trent Hauck
// All Rights Reserved

use noodles::fasta;
use noodles::fastq;
use noodles::gff;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::str;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FastaRecord {
    pub id: String,
    pub description: Option<String>,
    pub sequence: String,
}

impl From<fasta::Record> for FastaRecord {
    fn from(src: fasta::Record) -> FastaRecord {
        let seq = src.sequence();

        let ss = str::from_utf8(&seq.as_ref()).unwrap();

        FastaRecord {
            id: src.name().to_string(),
            description: src.description().map_or(None, |i| Some(i.to_string())),
            sequence: String::from(ss),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FastqRecord {
    pub id: String,
    pub description: Option<String>,
    pub sequence: String,
    pub quality: String,
}

impl From<fastq::Record> for FastqRecord {
    fn from(src: fastq::Record) -> FastqRecord {
        let seq = src.sequence();

        let ss = str::from_utf8(&seq.as_ref()).unwrap();

        let noodles_quality = str::from_utf8(src.quality_scores()).unwrap();
        let name = str::from_utf8(src.name()).unwrap();

        FastqRecord {
            id: String::from(name),
            description: None,
            sequence: String::from(ss),
            quality: String::from(noodles_quality),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GffRecord {
    pub seqname: String,
    pub source: String,
    pub feature: String,
    pub start: usize,
    pub end: usize,
    pub score: Option<f32>,
    pub strand: String,
    pub frame: Option<String>,
    pub attribute: HashMap<String, String>,
}

impl From<gff::Record> for GffRecord {
    fn from(src: gff::Record) -> GffRecord {
        let seqname = src.reference_sequence_name();
        let source = src.source();
        let feature_type = src.ty();
        let start = src.start();
        let end = src.end();
        let score = src.score();
        let strand = src.strand();
        let phase = src.phase().map_or(None, |f| Some(f.to_string()));

        let mut gff_attrs = HashMap::<String, String>::new();

        for i in src.attributes().iter() {
            let k = String::from(i.key());
            let v = String::from(i.value());

            // TODO: this isn't faithful to GFF3, value should be a vector
            if gff_attrs.contains_key(&k) {
                continue;
            } else {
                gff_attrs.insert(k, v);
            }
        }

        GffRecord {
            seqname: String::from(seqname),
            source: String::from(source),
            feature: String::from(feature_type),
            start: usize::from(start),
            end: usize::from(end),
            score,
            strand: String::from(strand.as_ref()),
            frame: phase,
            attribute: gff_attrs,
        }
    }
}
