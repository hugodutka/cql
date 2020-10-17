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
    pub async fn new(addr: impl ToSocketAddrs) -> Result<Connection> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|err| Error::DatabaseConnectionFailed(err))?;

        let mut conn = Connection { stream };

        conn.send_frame(Frame::from(Startup {})).await?;

        Ok(conn)
    }

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
