use crate::result::Result;

use log::debug;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use imap_codec::{command::Command, core::Tag};

pub enum CommandPipe<'a> {
    // next and prev command
    Next(Command<'a>, Option<Command<'a>>),
    Noop,
    Quit,
}

pub struct IMAPServ<'a, IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    socket: &'a mut IO,
    buf: Vec<u8>,
}

impl<'a, IO> IMAPServ<'a, IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(socket: &'a mut IO) -> Self {
        Self {
            socket,
            buf: Vec::new(),
        }
    }

    pub async fn read(&mut self) -> Result<&[u8]> {
        let mut buf = [0; 1024];
        let n = self.socket.read(&mut buf).await?;
        self.buf.extend_from_slice(&buf[..n]);
        Ok(&self.buf)
    }

    pub async fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.socket.write_all(buf).await?;
        Ok(())
    }

    pub async fn write_str(&mut self, buf: &str) -> Result<()> {
        self.socket.write_all(buf.as_bytes()).await?;
        Ok(())
    }
    pub async fn write_strln(&mut self, buf: &str) -> Result<()> {
        self.socket.write_all(buf.as_bytes()).await?;
        self.socket.write_all(b"\r\n").await?;
        Ok(())
    }

    pub async fn ok(&mut self, tag: &str, msg: &str) -> Result<()> {
        debug!(":> {} {}", tag, msg);
        self.write_str(&format!("{} OK {}\r\n", tag, msg)).await
    }

    pub async fn ok_completed(&mut self, tag: &Tag<'_>, cmd: &str) {
        self.ok_completed2(tag.as_ref(), cmd).await;
    }

    pub async fn ok_completed2(&mut self, tag: &str, cmd: &str) {
        Self::mon_result(self.ok(tag, &format!("{} completed", cmd)).await);
    }

    fn mon_result(result: Result<()>) {
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to write to socket; err = {:?}", e);
            }
        }
    }

    /**
     * Send status message to client.
     */
    pub async fn status(&mut self, msg: &str) {
        debug!("status: {}", msg);
        Self::mon_result(self.write_str(&format!("* {}\r\n", msg)).await)
    }
}
