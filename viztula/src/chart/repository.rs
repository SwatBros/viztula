use sql_builder::{dname, quote, SqlBuilder, SqlName};
use sqlx::{PgPool, Row};

use crate::utils::{list, serialize_json};

use super::model::Chart;

pub async fn insert(pool: &PgPool, chart: Chart) -> Result<i64, Box<dyn std::error::Error>> {
    let sql = SqlBuilder::insert_into("chart")
        .field(dname!("table"))
        .field(dname!("columns"))
        .values(&[&quote(chart.table), &list(&chart.columns)])
        .returning("id")
        .sql()?;

    let row = sqlx::query(&sql).fetch_one(pool).await?;
    Ok(row.get("id"))
}

pub async fn query(pool: &PgPool, query: String) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let rows = sqlx::query(&query).fetch_all(pool).await?;
    Ok(serialize_json(rows)?)
}
