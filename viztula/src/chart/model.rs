use serde::Deserialize;

#[derive(Deserialize)]
pub struct Chart {
    pub db: String,
    pub table: String,
    pub columns: Vec<String>,
    pub group_by: Vec<String>,
    pub order_by: Vec<String>,
    pub limit: Option<u32>,
}

pub trait Query {
    fn query(&self) -> String;
}

impl Query for Chart {
    fn query(&self) -> String {
        format!(
            "SELECT {} FROM {} GROUP BY {} ORDER BY {} {}",
            self.columns.join(", "),
            self.table,
            self.group_by.join(", "),
            self.order_by.join(", "),
            self.limit.map(|limit| format!("LIMIT {}", limit)).unwrap_or_default()
        )
    }
}
