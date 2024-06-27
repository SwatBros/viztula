use sql_builder::{dname, quote, SqlBuilder, SqlName};
use tokio_postgres::{NoTls, Row};

use crate::utils::list;

use super::model::Chart;

pub async fn insert(chart: Chart) -> Result<i64, Box<dyn std::error::Error>> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
            .await
            .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let sql = SqlBuilder::insert_into("chart")
        .field(dname!("table"))
        .field(dname!("columns"))
        .values(&[&quote(chart.table), &list(&chart.columns)])
        .returning("id")
        .sql()?;

    let row = client.query_one(&sql, &[]).await?;
    Ok(row.get("id"))
}

pub async fn query(query: String) -> Result<Vec<Row>, tokio_postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
            .await
            .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.query(&query, &[]).await
}
