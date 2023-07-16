pub use crate::imap_serv::{CommandPipe, IMAPServ};
use crate::result::Result;
use anyhow::anyhow;
use imap_codec::command::CommandBody;

use log::debug;

use tokio::io::{AsyncRead, AsyncWrite};

use crate::handlers::*;
use imap_codec::{codec::Decode, command::Command};

pub async fn process_command<'a, 'b, IO>(
    buf: &'a [u8],
    socket: &'b mut IO,
) -> Result<CommandPipe<'a>>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    let mut imap_sock = IMAPServ::new(socket);

    // if only CRLF ignore
    if buf.len() == 2 && buf[0] == 13 && buf[1] == 10 {
        return Ok(CommandPipe::Noop);
    }

    let cmd = command_decode(buf)?;

    debug!(":< {}", &cmd.body.name());

    match cmd.body.clone() {
        CommandBody::Noop => NoopHandler::handle(&mut imap_sock, &cmd).await,
        CommandBody::List {
            reference,
            mailbox_wildcard,
        } => {
            ListHandler::handle(
                &mut imap_sock,
                &cmd,
                reference,
                mailbox_wildcard,
            )
            .await
        }
        CommandBody::Select { mailbox } => {
            SelectHandler::handle(&mut imap_sock, &cmd, mailbox).await
        }
        CommandBody::Login { username, password } => {
            LoginHandler::handle(&mut imap_sock, &cmd, username, password).await
        }
        CommandBody::Capability => {
            CapabilityHandler::handle(&mut imap_sock, &cmd).await
        }
        CommandBody::Search {
            charset,
            criteria,
            uid,
        } => {
            SearchHandler::handle(&mut imap_sock, &cmd, charset, criteria, uid)
                .await
        }
        CommandBody::Logout => {
            LogoutHandler::handle(&mut imap_sock, &cmd).await
        }
        CommandBody::Fetch {
            sequence_set,
            macro_or_item_names,
            uid,
        } => {
            FetchHandler::handle(
                &mut imap_sock,
                &cmd,
                sequence_set,
                macro_or_item_names,
                uid,
            )
            .await
        }
        _ => Err(anyhow!("Invalid command body for {}", cmd.name()).into()),
    }
}

pub fn command_decode(buf: &[u8]) -> Result<Command> {
    let (_remainder, parsed) = Command::decode(buf)?;
    Ok(parsed)
}
