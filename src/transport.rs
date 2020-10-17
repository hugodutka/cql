use crate::error::{Error, Result};
use crate::frame::{Frame, Startup};
use crate::query::Query;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::prelude::*;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Establishes a new connection to the database and sends a startup frame.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cql::Connection;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut conn = Connection::new("localhost:9042").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(addr: impl ToSocketAddrs) -> Result<Connection> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|err| Error::DatabaseConnectionFailed(err))?;

        let mut conn = Connection { stream };

        conn.send_frame(Frame::from(Startup {})).await?;

        Ok(conn)
    }

    /// Queries the database. Does not fetch the query's result; only checks if it was delivered.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cql::{Connection, Query};
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut conn = Connection::new("localhost:9042").await?;
    /// let query = Query::new("INSERT INTO db.some_table(a) VALUES (1)");
    /// conn.query(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query(&mut self, query: Query) -> Result<()> {
        self.send_frame(Frame::from(query))
            .await
            .map_err(|err| Error::QueryNotSent(Box::new(err)))?;

        Ok(())
    }

    async fn send_frame(&mut self, frame: Frame) -> Result<()> {
        self.stream
            .write_all(frame.as_bytes().as_slice())
            .await
            .map_err(|err| Error::FrameNotSent(err))?;

        Ok(())
    }
}
