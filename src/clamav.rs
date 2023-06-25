use std::borrow::Borrow;

use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct ClamAvClient {
    socket: TcpStream,
}

/// Scan result
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Scan {
    Safe,
    Unsafe(String),
}

/// INIT socket buffer: nINSTREAM\n
const INIT_SOCKET_BUF: &str = "nINSTREAM\n";
/// FOUND ascii in response
const FOUND_RESPONSE: &str = "FOUND";

impl ClamAvClient {
    /// instantiate a new `ClamAvClient`
    pub async fn init(address: &str) -> anyhow::Result<Self> {
        let socket = TcpStream::connect(address).await?;
        info!("connection with {} established.", address);

        let mut client = Self { socket };

        client.write(INIT_SOCKET_BUF.as_bytes()).await?;
        Ok(client)
    }

    /// Scan data with ClamAV contained in reader
    pub async fn scan(
        &mut self,
        mut reader: impl AsyncReadExt + Unpin,
        size: u32,
    ) -> anyhow::Result<Scan> {
        self.write_filesize(size).await?;
        loop {
            let mut buffer = vec![0; 4096];
            let count = reader.read(&mut buffer).await?;
            if count == 0 {
                break;
            }
            self.write(&buffer[..count]).await?;
        }

        self.finish().await
    }

    /// write file size.
    /// This function must be called before any `send`
    async fn write_filesize(&mut self, size: u32) -> anyhow::Result<()> {
        let mut buf = vec![];
        buf.put_u32(size);

        self.write(&buf).await
    }

    /// finish send op and wait for scan result
    async fn finish(&mut self) -> anyhow::Result<Scan> {
        // send terminator
        let buf = BytesMut::zeroed(4);
        self.write(&buf).await?;

        let mut buf = Vec::with_capacity(4096);
        self.socket.read_buf(&mut buf).await?;

        debug!("ClamAV IN: {:?}", buf);

        let buf = String::from_utf8_lossy(&buf);

        debug!("ClamAV IN: {buf}");

        if buf.contains(FOUND_RESPONSE) {
            Ok(Scan::Unsafe(Self::threat_name(buf.borrow())))
        } else {
            Ok(Scan::Safe)
        }
    }

    async fn write(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        info!("ClamAV OUT {} bytes", buf.len());
        debug!("ClamAV OUT: {:?}", buf);
        self.socket.write(buf).await?;

        Ok(())
    }

    /// get revealed threat name
    fn threat_name(buf: &str) -> String {
        buf.replace("stream:", "")
            .trim()
            .replace("FOUND", "")
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_threat_name() {
        assert_eq!(
            ClamAvClient::threat_name("stream: pippo lippo FOUND").as_str(),
            "pippo lippo"
        );
    }
}
