mod frame;
mod query;
mod transport;
mod types;

pub use query::Query;
pub use transport::Connection;

/// Note: these tests expect a CQL compatible database to be available on `localhost:9042`.
#[cfg(test)]
mod tests {
    use super::*;

    const DATABASE_URL: &str = "localhost:9042";

    #[tokio::test]
    async fn can_initialize_connection() {
        Connection::new(DATABASE_URL).await.unwrap();
    }

    #[tokio::test]
    async fn can_send_query() {
        let mut conn = Connection::new(DATABASE_URL).await.unwrap();
        let query = Query::new("INSERT INTO ks.t(a,b,c) VALUES (1,2,'abc')");
        conn.query(query).await.unwrap();
    }
}
