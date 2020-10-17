pub struct Query {
    pub text: String,
}

impl Query {
    pub fn new(query: &str) -> Query {
        Query {
            text: query.to_string(),
        }
    }
}
