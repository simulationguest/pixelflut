use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub struct Canvas {
    addr: String,
}

pub struct Coordinates {
    x: u32,
    y: u32,
}

impl Canvas {
    pub async fn get_size(&self) -> Result<Coordinates> {
        let stream = TcpStream::connect(&self.addr).await?;
        let mut stream = BufReader::new(stream);

        stream.write_all(b"SIZE\n").await?;

        let mut line = String::new();
        stream.read_line(&mut line).await?;

        let mut parts = line.split(" ").skip(1);

        let width = parts.next().unwrap().parse()?;
        let height = parts.next().unwrap().parse()?;

        Ok(Coordinates {
            x: width,
            y: height,
        })
    }
}
