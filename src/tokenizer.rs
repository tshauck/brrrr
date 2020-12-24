use std::io::{Read, Result};

use bio::io::fasta;
use tokenizers::{models::bpe::{BpeTrainerBuilder, BPE}, normalizers::{NFC, Sequence, Strip}, processors::byte_level::ByteLevel};
use tokenizers::TokenizerBuilder;

pub fn train_tokenizer<R: Read + Send>(input: R) -> Result<()> {
    let mut trainer = BpeTrainerBuilder::new()
        .show_progress(true)
        .vocab_size(50)
        .build();

    let mut tokenizer = TokenizerBuilder::new()
        .with_pre_tokenizer(Some(ByteLevel::default()))
        .with_model(BPE::default())
        .with_normalizer(Some(Sequence::new(vec![
            Strip::new(true, true).into(),
            NFC.into(),
        ])))
        .with_post_processor(Some(ByteLevel::default()))
        .with_decoder(Some(ByteLevel::default()))
        .build()
        .unwrap();

    let reader = fasta::Reader::new(input);

    let records = reader.records().map(|r| r.unwrap().seq().to_owned());
    let strs = records.map(|f| std::str::from_utf8(f[..].as_ref()).unwrap().to_owned());
    
    let trained = tokenizer.train(&mut trainer, strs);
    trained.unwrap().save("tokenizer.json", true);

    Ok(())
}
