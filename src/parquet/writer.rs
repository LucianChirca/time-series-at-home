use crate::parquet::ParquetConfig;
use arrow::array::record_batch;
use parquet::arrow::arrow_writer::ArrowWriter;
use std::fs::File;

pub struct ParquetWriter {
    config: ParquetConfig,
}

impl ParquetWriter {
    pub fn new(config: ParquetConfig) -> Self {
        Self { config }
    }

    pub fn write(&self, value: f64, timestamp: i64) -> Result<(), Box<dyn std::error::Error>> {
        let to_write = record_batch!(
            ("value", Float64, [value]),
            ("timestamp", Int64, [timestamp])
        ).unwrap();

        let file = File::create(&self.config.file_path)?;
        let mut writer = ArrowWriter::try_new(file, to_write.schema(), None)?;

        writer.write(&to_write)?;
        writer.close()?;

        Ok(())
    }
}