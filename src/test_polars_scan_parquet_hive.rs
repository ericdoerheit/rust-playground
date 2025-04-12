use chrono::{Datelike, NaiveDate};
use fastrand::Rng;
use polars::error::to_compute_err;
use polars::io::HiveOptions;
use polars::prelude::ParquetWriter;
use polars::prelude::*;
use std::fs;
use std::iter::repeat_with;
use std::path::PathBuf;

const TEST_DIR_DATA: &str = "tmp/integration_tests/polars/data";

fn generate_and_save_test_data() {
    fastrand::seed(42);
    let mut rng = Rng::new();

    let base_directory = TEST_DIR_DATA.to_string();
    let source = 1;

    // Create directory for the data engine
    match fs::create_dir_all(TEST_DIR_DATA) {
        Ok(_) => println!("Successfully created directory: {}", TEST_DIR_DATA),
        Err(e) => println!("Error creating directory: {}", e),
    }

    let start_date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2021, 1, 3).unwrap();

    let mut date = start_date.clone() - chrono::Duration::days(1);
    while date <= end_date {
        date += chrono::Duration::days(1);

        let dir_path = format!("{}/source={}/year={}/month={}/day={}",
                               base_directory, source,
                               date.year(), date.month(), date.day());

        // Omit the creation of the data frames if the directory already exists
        if fs::metadata(dir_path.clone()).is_ok() {
            continue;
        } else {
            match fs::create_dir_all(dir_path.clone()) {
                Ok(_) => (),
                Err(e) => println!("Error creating directory: {}", e),
            }
        }

        let mut df = create_dataframe(&mut rng, date.clone());

        let timestamp = date.and_hms_opt(0, 0, 0).unwrap()
            .and_utc().timestamp();
        let file_path = format!("{}/{}.parquet", dir_path, timestamp);
        let file = fs::File::create(file_path).unwrap();
        ParquetWriter::new(file).with_row_group_size(Some(4096)).finish(&mut df).unwrap();
    }
}

fn create_dataframe(rng: &mut Rng, date: NaiveDate) -> DataFrame {
    let frame_interval_ms = 400;

    let start = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis();
    let end = date.and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp_millis();

    let timestamps: Vec<i64> = (start..end).filter(|x| *x % frame_interval_ms == 0).collect();

    let x: Vec<f32> = repeat_with(|| rng.f32()).take(timestamps.len()).collect();
    let y: Vec<f32> = repeat_with(|| rng.f32()).take(timestamps.len()).collect();
    let z: Vec<f32> = vec![0.0; timestamps.len()];

    df!(
        "timestamp" => &timestamps.clone(),
        "x" => &x.clone(),
        "y" => &y.clone(),
        "z" => &z.clone(),
    ).unwrap()
}

#[test]
fn test_polars_parquet_scan() {
    generate_and_save_test_data();

    let mut hive_options = HiveOptions::default();

    // Define the fields of the hive schema
    let fields = vec![
        Field::new("source".into(), DataType::UInt32),
        Field::new("year".into(), DataType::Int32),
        Field::new("month".into(), DataType::Int8),
        Field::new("day".into(), DataType::Int8),
    ];

    // Create the schema
    let schema = Schema::from_iter(fields.iter().map(|f| f.clone()));

    // Create a reference to the schema
    let schema_ref = SchemaRef::new(schema);
    hive_options.schema = Some(schema_ref);
    hive_options.enabled = Some(true);

    let mut scan_args = ScanArgsParquet::default();
    scan_args.hive_options = hive_options;

    let schema_path = format!("{}/*/*/*/*/*.parquet", TEST_DIR_DATA.to_string());
    let schema_path = PathBuf::from(schema_path);
    let path_str = schema_path.to_string_lossy();
    let paths = glob::glob(&path_str)
        .expect("Invalid glob pattern given");
    let paths = paths.map(|v| v.map_err(to_compute_err))
        .into_iter()
        .collect::<PolarsResult<Arc<[PathBuf]>>>().unwrap();

    let lf = LazyFrame::scan_parquet_files(paths.clone(), scan_args.clone()).unwrap();

    let lf = lf
        .filter(
            col("day").eq(lit(2))
                .and(col("x").gt(lit(0.7)))
                .and(col("y").lt(lit(0.5)))
                .and(col("z").eq(lit(0)))
        ).collect();

    if lf.is_err() {
        panic!("Error: {}", lf.err().unwrap());
    }

    let lf = lf.unwrap();
    lf.get_columns().iter().for_each(|s| println!("{} {}", s.name(), s.len()));

    println!("Result:\n{}", lf);
}
