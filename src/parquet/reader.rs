use crate::parquet::ParquetConfig;
use arrow::array::{Float64Array, Int64Array};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;
use std::error::Error;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TimeSeriesData {
    pub value: f64,
    pub timestamp: i64,
}

pub struct ParquetReader {
    config: ParquetConfig,
}

impl ParquetReader {
    pub fn new(config: ParquetConfig) -> Self {
        Self { config }
    }

    pub fn read(&self) -> Result<Vec<TimeSeriesData>, Box<dyn Error>> {
        let file = File::open(&self.config.file_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let mut reader = builder.build()?;

        let mut results = Vec::new();
        while let Some(batch) = reader.next() {
            let batch = batch?;
            let value_array = batch.column(0).as_any().downcast_ref::<Float64Array>().unwrap();
            let timestamp_array = batch.column(1).as_any().downcast_ref::<Int64Array>().unwrap();

            for i in 0..batch.num_rows() {
                results.push(TimeSeriesData {
                    value: value_array.value(i),
                    timestamp: timestamp_array.value(i),
                });
            }
        }

        Ok(results)
    }
} 