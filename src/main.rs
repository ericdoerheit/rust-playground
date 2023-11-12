#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod datatypes;

use std::any::Any;
use std::fmt::format;
use std::fs::File;
use std::ops::Deref;
use arrow::array::{Array, Float16Array, Float32Array, Float64Array, Int32Array, Int64Array, UInt16Array, UInt32Array, UInt64Array, UInt8Array};
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

    let fields = builder.schema().fields();
    for field in fields {
        let caption = format!("{} [{}]", field.name(), field.data_type());
        table_columns.push(caption);
        let col_data = Vec::new();
        table_data.push(col_data);
    }

    let mut reader = builder.build().unwrap();
    let record_batch = reader.next().unwrap().unwrap();
    println!("Read {} records.", record_batch.num_rows());

    // let _batch = reader.next().unwrap().unwrap();

    while let batch = reader.next() {
        match batch {
            Some(b) => {
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
                            let arr = column
                                .as_any()
                                .downcast_ref::<Int32Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::Int32(element.unwrap()));
                            }
                        }
                        ArrowDataType::Int64 => {
                            let arr = column
                                .as_any()
                                .downcast_ref::<Int64Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::Int64(element.unwrap()));
                            }
                        }
                        ArrowDataType::UInt8 => {
                            let arr = column
                                .as_any()
                                .downcast_ref::<UInt8Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::UInt8(element.unwrap()));
                            }
                        }
                        ArrowDataType::UInt16 => {
                            let arr = column
                                .as_any()
                                .downcast_ref::<UInt16Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::UInt16(element.unwrap()));
                            }
                        }
                        ArrowDataType::UInt32 => {
                            let arr = column
                                .as_any()
                                .downcast_ref::<UInt32Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::UInt32(element.unwrap()));
                            }
                        }
                        ArrowDataType::UInt64 => {
                            let arr = column
                                .as_any()
                                .downcast_ref::<UInt64Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::UInt64(element.unwrap()));
                            }
                        }
                        ArrowDataType::Float16 => {
                            let arr = column
                                .as_any()
                                .downcast_ref::<Float16Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::Float16(element.unwrap().to_f32()));
                            }
                        }
                        ArrowDataType::Float32 => {
                            let arr = column
                                .as_any()
                                .downcast_ref::<Float32Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::Float32(element.unwrap()));
                            }
                        }
                        ArrowDataType::Float64 => {
                            let arr = column
                                .as_any()
                                .downcast_ref::<Float64Array>()
                                .expect("Failed to downcast");
                            for element in arr {
                                table_data_col.push(DataType::Float64(element.unwrap()));
                            }
                        }
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
            my_app.table_data = table_data;
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
            // egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .auto_shrink([false, false])
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
                            18.0, self.table_data[0].len(),
                            |row_index, mut row| {

                                for i in 0..self.table_columns.len() {
                                    row.col(|ui| {
                                        // ui.label(row_index.to_string());
                                        ui.label(self.table_data[i][row_index].to_string());
                                    });
                                }
                            },
                        );
                    });
            // });
        });
    }
}