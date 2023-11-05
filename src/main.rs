#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod datatypes;

use std::any::Any;
use std::fmt::format;
use std::fs::File;
use std::ops::Deref;
use arrow::array::{Array, Int32Array};
use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};
use egui_extras::{Column, TableBuilder};
use log::{debug, error};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::datatypes::DataType as ArrowDataType;
use datatypes::DataType;


fn set_fonts(ctx: &egui::Context) {
    debug!("Load fonts");

    let mut fonts = FontDefinitions::default();
    // let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "IBM Plex Sans".to_owned(),
        FontData::from_static(include_bytes!("../fonts/IBMPlexSans-Regular.ttf")),
    );

    // match fonts.families.get_mut(&FontFamily::Proportional) {
    //     Some(font_families) => font_families.push("IBM Plex Sans".to_owned()),
    //     None => error!("Didn't find font family. That should not happen.")
    // }

    fonts.families.insert(FontFamily::Proportional, vec!["IBM Plex Sans".into()]);

    ctx.set_fonts(fonts);

    // let mut style = (*ctx.style()).clone();
    // style.text_styles = [
    //     (TextStyle::Body, FontId::new(21.0, FontFamily::Proportional)),
    //     (TextStyle::Button, FontId::new(18.0, FontFamily::Monospace)),
    //     (TextStyle::Monospace, FontId::new(18.0, FontFamily::Monospace)),
    //     (TextStyle::Small, FontId::new(16.0, FontFamily::Proportional)),
    //     (TextStyle::Heading, FontId::new(48.0, FontFamily::Proportional)),
    // ].into();
    // ctx.set_style(style);
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut table_data: Vec<Vec<DataType>> = Vec::new();
    let mut table_columns: Vec<String> = Vec::new();

    // TODO do not panic
    let file = File::open("data/2023-07-27T103728_items.parquet").unwrap();

    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();

    println!("Converted arrow schema is: {}", builder.schema());

    let fields = builder.schema().fields();
    for field in fields {
        println!("{}", field);
        println!("{}", field.data_type());

        let data_type = field.data_type();

        let caption = format!("{} [{}]", field.name(), field.data_type());
        table_columns.push(caption);

        let col_data = Vec::new();

        // DataType::Float32(50.5);

        // enum SpreadsheetCell {
        //     Int(i32),
        //     Float(f64),
        //     Text(String),
        // }
        // let row = vec![
        //     SpreadsheetCell::Int(3),
        //     SpreadsheetCell::Text(String::from("blue")),
        //     SpreadsheetCell::Float(10.12),
        // ];

        // let col_data = match data_type {
        //     // ArrowDataType::Null => Vec::<DataType::Null>::new(),
        //     // ArrowDataType::Boolean => Vec::<bool>::new(),
        //     // ArrowDataType::Boolean => Vec::<DataType::Boolean>::new(),
        //     ArrowDataType::Int8 => Vec::<DataType::Int8>::new(),
        //     ArrowDataType::Int16 => Vec::<DataType::Int16>::new(),
        //     ArrowDataType::Int32 => Vec::<DataType::Int32>::new(),
        //     ArrowDataType::Int64 => Vec::<DataType::Int64>::new(),
        //     ArrowDataType::UInt8 => Vec::<DataType::UInt8>::new(),
        //     ArrowDataType::UInt16 => Vec::<DataType::UInt16>::new(),
        //     ArrowDataType::UInt32 => Vec::<DataType::UInt32>::new(),
        //     ArrowDataType::UInt64 => Vec::<DataType::UInt64>::new(),
        //     ArrowDataType::Float16 => Vec::<DataType::Float16>::new(),
        //     ArrowDataType::Float32 => Vec::<DataType::Float32>::new(),
        //     ArrowDataType::Float64 => Vec::<DataType::Float64>::new(),
        //     // ArrowDataType::Timestamp(_, _) => Vec::<DataType::Timestamp>::new(),
        //     // ArrowDataType::Date32 => Vec::<DataType::Date32>::new(),
        //     // ArrowDataType::Date64 => Vec::<DataType::Date64>::new(),
        //     // ArrowDataType::Time32(_) => Vec::<DataType::Time32>::new(),
        //     // ArrowDataType::Time64(_) => Vec::<DataType::Time64>::new(),
        //     // ArrowDataType::Duration(_) => Vec::<DataType::Duration>::new(),
        //     // ArrowDataType::Interval(_) => Vec::<DataType::Interval>::new(),
        //     // ArrowDataType::Binary => Vec::<DataType::Binary>::new(),
        //     // ArrowDataType::FixedSizeBinary(_) => Vec::<DataType::FixedSizeBinary()>::new(),
        //     // ArrowDataType::LargeBinary => Vec::<DataType::LargeBinary>::new(),
        //     ArrowDataType::Utf8 => Vec::<DataType::Utf8>::new(),
        //     // ArrowDataType::LargeUtf8 => Vec::<DataType::LargeUtf8>::new(),
        //     // ArrowDataType::List(_) => Vec::<(_)>::new(),
        //     // ArrowDataType::FixedSizeList(_, _) => Vec::<DataType::FixedSizeList>::new(),
        //     // ArrowDataType::LargeList(_) => Vec::<DataType::LargeList>::new(),
        //     // ArrowDataType::Struct(_) => Vec::<DataType::Struct>::new(),
        //     // ArrowDataType::Union(_, _) => Vec::<DataType::Union>::new(),
        //     // ArrowDataType::Dictionary(_, _) => Vec::<DataType::Dictionary>::new(),
        //     // ArrowDataType::Decimal128(_, _) => Vec::<DataType::Decimal128>::new(),
        //     // ArrowDataType::Decimal256(_, _) => Vec::<DataType::Decimal256>::new(),
        //     // ArrowDataType::Map(_, _) => Vec::<DataType::Map>::new(),
        //     // ArrowDataType::RunEndEncoded(_, _) => Vec::<DataType::RunEndEncoded>::new(),
        //     _ => Vec::<String>::new()
        // };


        /*        match number {
                    // Match a single value
                    1 => println!("One!"),
                    // Match several values
                    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
                    // Match an inclusive range
                    13..=19 => println!("A teen"),
                    // Handle the rest of cases
                    _ => println!("Ain't special"),
                }*/

        // let col_data = Vec::new();
        table_data.push(col_data);
    }

    let mut reader = builder.build().unwrap();
    let record_batch = reader.next().unwrap().unwrap();
    println!("Read {} records.", record_batch.num_rows());

    // let _batch = reader.next().unwrap().unwrap();

    while let batch = reader.next() {
        match batch {
            Some(b) => {
                // println!("Read {} records.", r.unwrap().schema());
                // println!("Read {} records.", b.unwrap().num_rows());
                let b = b.unwrap();
                let num_columns = b.num_columns();
                let columns = b.columns();
                for i in 0..num_columns {
                    let column = &columns[i];
                    let mut table_data_col = &mut table_data[i];

                    let dt = column.data_type();
                    match dt {
                        ArrowDataType::Boolean => {}
                        ArrowDataType::Int8 => {}
                        ArrowDataType::Int16 => {}
                        ArrowDataType::Int32 => {
                            let int32array = column
                                .as_any()
                                .downcast_ref::<Int32Array>()
                                .expect("Failed to downcast");
                            for element in int32array {
                                table_data_col.push(DataType::Int32(element.unwrap()));
                            }
                        }
                        ArrowDataType::Int64 => {}
                        ArrowDataType::UInt8 => {}
                        ArrowDataType::UInt16 => {}
                        ArrowDataType::UInt32 => {}
                        ArrowDataType::UInt64 => {}
                        ArrowDataType::Float16 => {}
                        ArrowDataType::Float32 => {}
                        ArrowDataType::Float64 => {}
                        _ => {}
                    }

                    // println!("Read {} records.", b.unwrap().column(0).as_any().downcast_ref::<Int32Array>());
                    // let int32array = batch
                    //     .column(0)
                    //     .as_any()
                    //     .downcast_ref::<Int32Array>()
                    //     .expect("Failed to downcast");
                }

                for column in b.columns() {}
            }
            None => {
                break;
            }
        }
        // let record_batch = result.unwrap();
    }

    eframe::run_native(
        "Parquet Reader",
        options,
        Box::new(|cc| {
            set_fonts(&cc.egui_ctx);

            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut my_app = Box::<MyApp>::default();
            // my_app.table_data = table_data;
            my_app.table_columns = table_columns;
            my_app
        }),
    )
}

struct MyApp {
    table_columns: Vec<String>,
    table_data: Vec<Vec<DataType>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            table_columns: Vec::new(),
            table_data: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    // .column(Column::auto())
                    // .column(Column::initial(100.0).range(40.0..=300.0))
                    // .column(Column::initial(100.0).at_least(40.0).clip(true))
                    // .column(Column::remainder())
                    .min_scrolled_height(0.0);

                for _ in &self.table_columns {
                    table = table.column(Column::auto());
                }

                table
                    .header(20.0, |mut header| {
                        for column in &self.table_columns {
                            header.col(|ui| {
                                ui.strong(column.clone());
                            });
                        }
                    })
                    .body(|mut body| {
                        body.rows(
                            18.0, self.table_data.len(),
                            |row_index, mut row| {
                                row.col(|ui| {
                                    ui.label(row_index.to_string());
                                });
                                row.col(|ui| {
                                    // ui.label(format!("Data {}", self.table_data[row_index % self.table_data.len()]));
                                });
                                row.col(|ui| {
                                    ui.label("This row has some long text that you may want to clip, or it will take up too much horizontal space!");
                                });
                                row.col(|ui| {
                                    ui.style_mut().wrap = Some(false);
                                    ui.label("Normal row");
                                });
                            },
                        );
                    });
            });
        });
    }
}