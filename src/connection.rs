use anyhow::Result;
use core::str;
use std::fmt::Write;

use crate::{color::Color, coordinates::Coordinates};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

const LINE_SIZE: usize = 32;

/// An allocation-free writable buffer
struct LineBuf {
    inner: [u8; LINE_SIZE],
    pos: usize,
}

impl LineBuf {
    fn new() -> Self {
        Self {
            inner: [0; LINE_SIZE],
            pos: 0,
        }
    }

    fn get_contents(&self) -> &[u8] {
        &self.inner[0..self.pos]
    }

    fn clear(&mut self) {
        self.pos = 0;
    }
}

impl Write for LineBuf {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let slice = &mut self.inner[self.pos..self.pos + s.len()];
        slice.copy_from_slice(s.as_bytes());
        self.pos += s.len();
        Ok(())
    }
}

/// Writing primitive for the canvas.
///
/// Used by the painter to draw images
pub struct Connection {
    stream: TcpStream,
    line_buf: LineBuf,
}

impl Connection {
    pub async fn new(addr: &str) -> Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(addr).await?,
            line_buf: LineBuf::new(),
        })
    }

    pub async fn get_size(&mut self) -> Result<Coordinates> {
        let mut stream = BufReader::new(&mut self.stream);

        stream.write_all(b"SIZE\n").await?;

        let mut line = String::new();
        stream.read_line(&mut line).await?;

        let mut parts = line.trim().split(" ").skip(1);

        let width = parts.next().unwrap().parse()?;
        let height = parts.next().unwrap().parse()?;

        Ok(Coordinates {
            x: width,
            y: height,
        })
    }

    pub async fn write_pixel(&mut self, coordinates: Coordinates, color: Color) -> Result<()> {
        let stream = &mut self.stream;
        let line_buf = &mut self.line_buf;

        line_buf.clear();
        write!(line_buf, "PX {coordinates} {color}\n")?;

        stream.write_all(line_buf.get_contents()).await?;
        Ok(())
    }
}

/// Manages a pool of connections. When the pool is empty, a new connection is created
pub struct Pool {
    addr: String,
    connections: Vec<Connection>,
}

impl Pool {
    pub fn new(addr: String) -> Pool {
        Self {
            addr,
            connections: Vec::new(),
        }
    }

    pub async fn get_size(&mut self) -> Result<Coordinates> {
        let mut conn = self.get().await?;
        let size = conn.get_size().await?;
        self.put(conn);
        Ok(size)
    }

    pub async fn get(&mut self) -> Result<Connection> {
        if self.connections.len() == 0 {
            println!("New connection");
            return Connection::new(&self.addr).await;
        }
        Ok(self.connections.pop().unwrap())
    }

    pub fn put(&mut self, writer: Connection) {
        self.connections.push(writer);
    }
}
