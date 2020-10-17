use crate::frame::{Frame, Startup};
use crate::query::Query;
use anyhow::{Context, Result};
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
            .context("failed to connect to the database")?;

        let mut conn = Connection { stream };

        conn.send_frame(Frame::from(Startup {}))
            .await
            .context("failed to send a startup frame")?;

        Ok(conn)
    }

    pub async fn query(&mut self, query: Query) -> Result<()> {
        self.send_frame(Frame::from(query)).await?;

        Ok(())
    }

    async fn send_frame(&mut self, frame: Frame) -> Result<()> {
        self.stream.write_all(frame.as_bytes().as_slice()).await?;

        Ok(())
    }
}
