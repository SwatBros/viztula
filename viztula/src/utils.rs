use std::collections::HashMap;

use log::warn;
use serde_json::json;
use sqlx::{
    postgres::PgRow,
    Column, Row, TypeInfo
};

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
                            "BOOL" => json!(row.get::<bool, _>(ordinal)),
                            "CHAR" => json!(row.get::<i8, _>(ordinal)),
                            "SMALLINT" | "SMALLSERIAL" | "INT2" => {
                                json!(row.get::<i16, _>(ordinal))
                            }
                            "INT" | "SERIAL" | "INT4" => json!(row.get::<i32, _>(ordinal)),
                            "BIGINT" | "BIGSERIAL" | "INT8" => json!(row.get::<i64, _>(ordinal)),
                            "REAL" | "FLOAT4" => json!(row.get::<f32, _>(ordinal)),
                            "DOUBLE PRECISION" | "FLOAT8" => json!(row.get::<f64, _>(ordinal)),
                            "VARCHAR" | "CHAR(N)" | "TEXT" | "NAME" | "CITEXT" => {
                                json!(row.get::<String, _>(ordinal))
                            },
                            "BYTEA" => json!(row.get::<Vec<u8>, _>(ordinal)),
                            "VOID" => json!(row.get::<(), _>(ordinal)),
                            "NUMERIC" => json!(row.get::<sqlx::types::BigDecimal, _>(ordinal)),
                            "TIMESTAMPTZ" => json!(row.get::<chrono::DateTime<chrono::Utc>, _>(ordinal)),
                            "TIMESTAMP" => json!(row.get::<chrono::NaiveDateTime, _>(ordinal)),
                            "DATE" => json!(row.get::<chrono::NaiveDate, _>(ordinal)),
                            "TIME" => json!(row.get::<chrono::NaiveTime, _>(ordinal)),
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
