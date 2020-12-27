use std::io::{Read, Result};

use bio::io::fasta;
use serde::{Deserialize, Serialize};
use tokenizers::{PreTokenizedString, PreTokenizer, models::{bpe::BpeTrainerBuilder, wordlevel::WordLevel}, normalizers::{Sequence, Strip, NFC}, processors::byte_level::ByteLevel};
use tokenizers::{Tokenizer, TokenizerBuilder};

use tokenizers::models::bpe::BPE;

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
pub struct KmerPreTokenizer {
    k: i32,
}

impl Default for KmerPreTokenizer {
    fn default() -> Self {
        Self { k: 1 }
    }
}

impl PreTokenizer for KmerPreTokenizer {
    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        pretokenized.split(|i, x| {
            x.split(
                |f: char| {
                    println!("{:?}", f);
                    true
                },
                tokenizers::SplitDelimiterBehavior::Isolated,
            )
        })
    }
}

pub fn train_tokenizer<R: Read + Send>(input: R) -> Result<()> {
    let mut trainer = BpeTrainerBuilder::new()
        .show_progress(true)
        .vocab_size(50)
        .build();

    // let mut tokenizer = TokenizerBuilder::new()
    //     // .with_pre_tokenizer(Some(ByteLevel::default()))
    //     .with_pre_tokenizer(Some(KmerPreTokenizer::default()))
    //     .with_model(WordLevel::default())
    //     .with_normalizer(Some(Sequence::new(vec![
    //         Strip::new(true, true).into(),
    //         NFC.into(),
    //     ])))
    //     .build()
    //     .unwrap();

    // let reader = fasta::Reader::new(input);

    // let records = reader.records().map(|r| r.unwrap().seq().to_owned());
    // let strs = records.map(|f| std::str::from_utf8(f[..].as_ref()).unwrap().to_owned());

    // let trained = tokenizer.train(&mut trainer, strs);
    // trained.unwrap().save("tokenizer.json", true);

    // let s = Tokenizer::from_file("./tokenizer.json").unwrap();

    // let encoding = s.encode("SEQVENCE", false).unwrap();

    // let encoding = tokenizer.encode("SEQVEN CE", false).unwrap();
    // println!("{:?}", encoding.get_tokens());

    Ok(())
}
