use std::collections::HashMap;

use log::warn;
use serde_json::json;
use sqlx::{postgres::PgRow, Column, Row, TypeInfo};

pub fn list<T: ToString>(list: &Vec<T>) -> String {
    format!(
        "'{{{}}}'",
        list.into_iter()
            .map(|v| format!("\"{}\"", v.to_string()))
            .collect::<Vec<String>>()
            .join(", ")
    )
}

pub fn serialize_json(rows: Vec<PgRow>) -> Result<Vec<serde_json::Value>, serde_json::Error> {
    let data: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!(row
                .columns()
                .into_iter()
                .map(|column| {
                    let ordinal = column.ordinal();
                    let type_name = column.type_info().name();
                    (
                        column.name(),
                        match type_name {
                            "TEXT" => json!(row.get::<String, _>(ordinal)),
                            "INTEGER" => json!(row.get::<i64, _>(ordinal)),
                            "BOOLEAN" => json!(row.get::<bool, _>(ordinal)),
                            "REAL" => json!(row.get::<f64, _>(ordinal)),
                            "TIMESTAMP" => json!(row.get::<chrono::NaiveDateTime, _>(ordinal)),
                            // probably missed a few other types?
                            _ => {
                                warn!("UNPROCESSED TYPE '{}'", type_name);
                                json!(format!("UNPROCESSED TYPE '{}'", type_name))
                            }
                        },
                    )
                })
                .collect::<HashMap<_, _>>())
        })
        .collect();

    Ok(data)
}
