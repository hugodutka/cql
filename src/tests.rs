/// Note: these tests expect a CQL compatible database to be available on `localhost:9042`. They
/// also expect that one will not be present at `localhost:14141`.
use crate::*;

const DATABASE_URL: &str = "localhost:9042";
const NONEXISTENT_DATABASE_URL: &str = "localhost:14141";

#[tokio::test]
async fn can_initialize_connection() {
    Connection::new(DATABASE_URL).await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn connection_initialization_fails_when_db_does_not_exist() {
    Connection::new(NONEXISTENT_DATABASE_URL).await.unwrap();
}

#[tokio::test]
async fn can_send_query() {
    let mut conn = Connection::new(DATABASE_URL).await.unwrap();
    let query = Query::new("INSERT INTO ks.t(a,b,c) VALUES (1,2,'abc')");
    conn.query(query).await.unwrap();
}

#[tokio::test]
async fn can_send_many_queries() {
    let mut conn = Connection::new(DATABASE_URL).await.unwrap();
    for i in 0..100u32 {
        let query = Query::new(format!("INSERT INTO ks.t(a,b,c) VALUES ({},2,'abc')", i).as_str());
        conn.query(query).await.unwrap();
    }
}
