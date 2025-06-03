use crate::parquet::ParquetConfig;
use std::sync::Arc;
use arrow::array::{Array, ArrayRef};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use std::fs::File;

pub struct ParquetWriter {
    config: ParquetConfig,
}

impl ParquetWriter {
    pub fn new(config: ParquetConfig) -> Self {
        Self { config }
    }

    pub fn write(&self, data: Arc<dyn Array + 'static>) -> Result<(), Box<dyn std::error::Error>> {
        let col = data as ArrayRef;
        let to_write = RecordBatch::try_from_iter([("value", col)]).unwrap();

        let file = File::create(&self.config.file_path)?;
        let mut writer = ArrowWriter::try_new(file, to_write.schema(), None)?;

        writer.write(&to_write)?;
        writer.close()?;

        Ok(())
    }
}