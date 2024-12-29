use super::{Buffer, Error};

use crate::{Color, Coordinates};

use std::{fmt::Write, str};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

/// Writing primitive for the canvas.
///
/// Used by the painter to draw images
///
/// In case you need multiple connections, use a Connection Pool instead
pub struct Connection {
    stream: TcpStream,
    buffer: Buffer,
}

impl Connection {
    async fn flush(&mut self) -> Result<(), Error> {
        self.stream.write_all(self.buffer.get_contents()).await?;
        self.buffer.clear();
        Ok(())
    }

    pub async fn new(addr: &str) -> Result<Self, Error> {
        Ok(Self {
            stream: TcpStream::connect(addr).await?,
            buffer: Buffer::new(),
        })
    }

    pub async fn get_canvas_size(&mut self) -> Result<Coordinates, Error> {
        let mut stream = BufReader::new(&mut self.stream);

        stream.write_all(b"SIZE\n").await?;

        let mut line = String::new();
        stream.read_line(&mut line).await?;

        let mut parts = line.trim().split(' ').skip(1);

        let width = parts.next().unwrap().parse()?;
        let height = parts.next().unwrap().parse()?;

        Ok(Coordinates {
            x: width,
            y: height,
        })
    }

    pub async fn set_offset(&mut self, offset: Coordinates) -> Result<(), Error> {
        writeln!(self.buffer, "OFFSET {offset}")?;
        self.flush().await
    }

    pub async fn write_pixel(
        &mut self,
        coordinates: Coordinates,
        color: Color,
    ) -> Result<(), Error> {
        writeln!(self.buffer, "PX {coordinates} {color}")?;
        self.flush().await
    }
}
