/// A query that can be executed with `cql::Connection::query`.
///
/// # Examples
///
/// ```
/// use cql::{Connection, Query};
///
/// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
/// let mut conn = Connection::new("localhost:9042").await?;
/// let query = Query::new("INSERT INTO db.some_table(a) VALUES (1)");
/// conn.query(query).await?;
/// # Ok(())
/// # }
/// ```
pub struct Query {
    pub text: String,
}

impl Query {
    /// Creates a new query. Does not validate it.
    pub fn new(query: &str) -> Query {
        Query {
            text: query.to_string(),
        }
    }
}
