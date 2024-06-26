use serde::Deserialize;

#[derive(Deserialize)]
pub struct Chart {
    pub table: String,
    pub columns: Vec<String>,
}

trait Query {
    fn query(&self) -> String;
}

impl Query for Chart {
    fn query(&self) -> String {
        format!("SELECT {} FROM {}", self.columns.join(", "), self.table)
    }
}

impl Chart {
    fn new(table: String, columns: Vec<String>) -> Self {
        Self { table, columns }
    }
}
