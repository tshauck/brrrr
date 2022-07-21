use arrow;
use parquet;
use std::io;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum BrrrrError {
    #[error("io error")]
    IOError(#[from] io::Error),

    #[error("arrow error")]
    ArrowError(#[from] arrow::error::ArrowError),

    #[error("parquet error")]
    ParquetError(#[from] parquet::errors::ParquetError),
}
