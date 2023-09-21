use std::env;

use polars::prelude::*;
use student_pairs::create_shuffled_series;

fn main() {
    // collect arguments [this file, input csv, output csv]
    let args: Vec<String> = env::args().collect();
    let file_in = &args[1];
    let file_out = &args[2];

    // Import students
    // single column csv with no header
    let students = CsvReader::from_path(file_in)
        .expect("File not read!")
        .infer_schema(None)
        .has_header(false)
        .finish()
        .unwrap();

    let nums = create_shuffled_series(&students);

    let out = students
        .clone()
        .lazy()
        .with_columns([nums.lit()])
        .with_columns_seq([
            // get group numbers (odds will be rounded down)
            ((col("x")) / lit(2)).alias("group"),
            // give each group member a number
            (col("x") % lit(2)).alias("z"),
        ])
        .collect()
        .unwrap();

    let mut out =
        pivot::pivot_stable(&out, ["column_1"], ["group"], ["z"], false, None, None).unwrap();

    let mut file_out = std::fs::File::create(file_out).unwrap();
    CsvWriter::new(&mut file_out).finish(&mut out).unwrap();
}

