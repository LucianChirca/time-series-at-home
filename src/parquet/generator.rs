use crate::parquet::ParquetConfig;
use std::sync::Arc;
use arrow::array::{Int64Array, ArrayRef};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;

pub struct ParquetGenerator {
    config: ParquetConfig,
}

impl ParquetGenerator {
    pub fn new(config: ParquetConfig) -> Self {
        Self { config }
    }

    pub fn generate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let col = Arc::new(Int64Array::from_iter_values([1, 2, 3])) as ArrayRef;
        let to_write = RecordBatch::try_from_iter([("col", col)]).unwrap();

        let mut buffer = Vec::new();
        let mut writer = ArrowWriter::try_new(&mut buffer, to_write.schema(), None).unwrap();

        writer.write(&to_write).unwrap();
        writer.close().unwrap();

        println!("{:?}", buffer);

        Ok(())
    }
}