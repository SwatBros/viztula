pub fn list<T: ToString>(list: &Vec<T>) -> String {
    format!(
        "'{{{}}}'",
        list.into_iter()
            .map(|v| format!("\"{}\"", v.to_string()))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
