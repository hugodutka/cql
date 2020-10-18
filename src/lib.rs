pub mod error;
mod frame;
mod query;
mod transport;
mod types;



pub use query::Query;
pub use transport::Connection;

/// Note: these tests expect a CQL compatible database to be available on `localhost:9042`.
#[cfg(test)]
mod tests {
  
    use std::time::Duration;
    use std::thread;
    use super::*;

    const DATABASE_URL: &str = "localhost:9042";

    use cdrs::{
        authenticators::NoneAuthenticator,
        cluster::{
          session::{
            new as new_session,
            Session,
          },
          ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool,
        },
        load_balancing::SingleNode,
        query::QueryExecutor,
        Result as CDRSResult,
      };
      
      type CurrentSession = Session<SingleNode<TcpConnectionPool<NoneAuthenticator>>>;
      
      fn create_db_session() -> CDRSResult<CurrentSession> {
        let auth = NoneAuthenticator;
        let node = NodeTcpConfigBuilder::new(DATABASE_URL, auth).build();
        let cluster_config = ClusterTcpConfig(vec![node]);
        new_session(&cluster_config, SingleNode::new())
      }

    
      
    fn prepare_test_db(table_name : &str) {
        let session = create_db_session()
          .expect("CDRS couldn't connect");
        session.query(format!("CREATE TABLE IF NOT EXISTS ks.{}(a int, b int, c text, primary key (a, b))", table_name))
          .expect("CDRS create table failed");
        session.query(format!("TRUNCATE ks.{}", table_name))
          .expect("CDRS delete from failed");
    }

    fn drop_test_db(table_name : &str) {
        let session = create_db_session()
          .expect("CDRS couldn't connect");
        session.query(format!("DROP TABLE IF EXISTS ks.{}", table_name)).expect("CDRS drop table failed");
    }
    
    #[tokio::test]
    async fn can_initialize_connection() {
        Connection::new(DATABASE_URL).await.unwrap();
    }

    #[tokio::test]
    async fn can_send_query() {
        const TABLE_NAME : &str = "test_query";
        prepare_test_db(TABLE_NAME);
        //thread::sleep(Duration::from_secs(20));
        let session = create_db_session().expect("CDRS couldn't connect");

        let count_rows = || -> usize {
            session.query(format!("SELECT * FROM ks.{} WHERE a=1 AND b=2 AND c='abc' ALLOW FILTERING;", TABLE_NAME))
            .expect("CDRS query")
            .get_body()
            .expect("CDRS get_body")
            .into_rows()
            .expect("CDRS into_rows")
            .len()};

        let mut conn = Connection::new(DATABASE_URL).await.unwrap();
        let query = Query::new(&format!("INSERT INTO ks.{}(a,b,c) VALUES (1,2,'abc')", TABLE_NAME));

        conn.query(query).await.unwrap();
        
        thread::sleep(Duration::from_secs(3));
        let row_count = count_rows();
        
        drop_test_db(TABLE_NAME);
        assert_eq!(1, row_count, "Bad insert");
    }

    #[tokio::test]
    async fn multiple_inserts() {
        const TABLE_NAME : &str = "test_multiple_inserts";
        const INSERT_NUM : usize = 10000;
        prepare_test_db(TABLE_NAME);
        let mut conn = Connection::new(DATABASE_URL).await.unwrap();

        let q = |a : usize, b : usize, c : &str| -> Query {
          Query::new(&format!("INSERT INTO ks.{}(a,b,c) VALUES ({},{},'{}')", TABLE_NAME, a, b, c))};
        
        for i in 1..INSERT_NUM + 1 {
          conn.query(q(i, i, "abc")).await.unwrap();
        }        

        thread::sleep(Duration::from_secs(3));

        let session = create_db_session().expect("CDRS couldn't connect");
        let row_len = session.query(format!("SELECT * FROM ks.{}", TABLE_NAME))
          .expect("CDRS select *")
          .get_body()
          .expect("get_body")
          .into_rows()
          .expect("into rows")
          .len();
        
        
        drop_test_db(TABLE_NAME);
        assert_eq!(row_len, INSERT_NUM);        
    }
}
