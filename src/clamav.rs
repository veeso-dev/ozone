use bytes::{BufMut, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct ClamAvClient {
    socket: TcpStream,
}

pub enum Scan {
    Safe,
    Unsafe,
}

/// INIT socket buffer: zINSTREAM\0
const INIT_SOCKET_BUF: [u8; 10] = [0x7a, 0x49, 0x4e, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4d, 0x00];
/// FOUND ascii in response
const FOUND_RESPONSE: &str = "FOUND";

impl ClamAvClient {
    /// instantiate a new `ClamAvClient`
    pub async fn init(address: &str) -> anyhow::Result<Self> {
        let socket = TcpStream::connect(address).await?;
        debug!("connection with {} established.", address);

        let mut client = Self { socket };

        client.write(&INIT_SOCKET_BUF).await?;
        Ok(client)
    }

    /// send data to stream
    pub async fn send(&mut self, bytes: Bytes) -> anyhow::Result<()> {
        let mut buf = BytesMut::zeroed(32);
        buf.put_u64(bytes.len() as u64);

        self.write(&buf).await?;
        self.write(&bytes).await?;

        Ok(())
    }

    /// finish send op and wait for scan result
    pub async fn finish(&mut self) -> anyhow::Result<Scan> {
        // send terminator
        let buf = BytesMut::zeroed(32);
        self.write(&buf).await?;

        let mut buf = String::default();
        self.socket.read_to_string(&mut buf).await?;

        trace!("ClamAV IN: {buf}");

        if buf.contains(FOUND_RESPONSE) {
            Ok(Scan::Unsafe)
        } else {
            Ok(Scan::Safe)
        }
    }

    async fn write(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        trace!("ClamAV OUT: {:?}", buf);
        self.socket.write_all(buf).await?;

        Ok(())
    }
}
