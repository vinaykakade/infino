//! `parquet_to_jsonl <file.parquet> <column> [max_rows]`: writes one
//! `{"text": ...}` line per row of a string column to stdout.
use std::io::{BufWriter, Write};

use arrow::array::{Array, AsArray};
use parquet::arrow::ProjectionMask;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max: usize = args
        .get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(usize::MAX);
    let file = std::fs::File::open(&args[1]).expect("open");
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).expect("parquet");
    let idx = builder.schema().index_of(&args[2]).expect("column");
    let mask = ProjectionMask::roots(builder.parquet_schema(), [idx]);
    let reader = builder.with_projection(mask).build().expect("reader");
    let mut out = BufWriter::new(std::io::stdout().lock());
    let mut n = 0usize;
    for batch in reader {
        let batch = batch.expect("batch");
        let col = batch.column(0);
        let mut emit = |s: &str| {
            serde_json::to_writer(&mut out, &serde_json::json!({ "text": s })).expect("w");
            out.write_all(b"\n").expect("w");
        };
        if let Some(a) = col.as_string_opt::<i32>() {
            for i in 0..a.len() {
                if n == max {
                    return;
                }
                if a.is_valid(i) {
                    emit(a.value(i));
                    n += 1;
                }
            }
        } else {
            let a = col.as_string::<i64>();
            for i in 0..a.len() {
                if n == max {
                    return;
                }
                if a.is_valid(i) {
                    emit(a.value(i));
                    n += 1;
                }
            }
        }
    }
}
