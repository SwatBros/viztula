use serde::Deserialize;

#[derive(Deserialize)]
pub struct Chart {
    pub table: String,
    pub columns: Vec<String>,
}

pub trait Query {
    fn query(&self) -> String;
}

impl Query for Chart {
    fn query(&self) -> String {
        format!("SELECT {} FROM {}", self.columns.join(", "), self.table)
    }
}