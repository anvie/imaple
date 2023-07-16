pub use crate::imap_serv::{CommandPipe, IMAPServ};
use crate::result::Result;
use anyhow::anyhow;
use imap_codec::command::CommandBody;

use log::debug;

use tokio::io::{AsyncRead, AsyncWrite};

use crate::cmd_handlers::*;
use imap_codec::{codec::Decode, command::Command};

// #[async_trait]
// trait CommandHandler<'a> {
//     async fn handle(&mut self, cmd: &Command<'a>) -> Result<Box<CommandPipe<'a>>>;
// }

// struct NoopHandler<'a, IO>
// where
//     IO: AsyncRead + AsyncWrite + Unpin,
// {
//     imap_sock: &'a mut IMAPServ<'a, IO>,
// }

// impl<'a, IO> NoopHandler<'a, IO>
// where
//     IO: AsyncRead + AsyncWrite + Unpin,
// {
//     pub fn new(imap_sock: &'a mut IMAPServ<'a, IO>) -> Self {
//         Self { imap_sock }
//     }
// }

// #[async_trait]
// impl<'a, IO> CommandHandler<'a> for NoopHandler<'a, IO>
// where
//     IO: AsyncRead + AsyncWrite + Unpin + Send,
// {
//     async fn handle(&mut self, cmd: &Command<'a>) -> Result<Box<CommandPipe<'a>>> {
//         debug!("NOOP");
//         self.imap_sock.status("23 EXISTS").await;
//         self.imap_sock.ok_completed(&cmd.tag, "NOOP").await;
//         Ok(Box::new(CommandPipe::Next(cmd.clone(), None)))
//     }
// }

// struct NoopHandler;
// impl NoopHandler {
//     pub async fn handle<'b, 'c, IO>(
//         imap_sock: &mut IMAPServ<'b, IO>,
//         cmd: &Command<'c>,
//     ) -> Result<CommandPipe<'c>>
//     where
//         IO: AsyncRead + AsyncWrite + Unpin,
//     {
//         imap_sock.status("23 EXISTS").await;
//         imap_sock.ok_completed(&cmd.tag, "NOOP").await;
//         Ok(CommandPipe::Next(cmd.clone(), None))
//     }
// }

pub async fn process_command<'a, 'b, IO>(
    buf: &'a [u8],
    socket: &'b mut IO,
) -> Result<CommandPipe<'a>>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    let mut imap_sock = IMAPServ::new(socket);

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
        _ => Err(anyhow!("Invalid command body for {}", cmd.name()).into()),
    }

    // match command_decode(buf) {
    //     Ok(cmd) => {
    //         match cmd.body.clone() {
    //             CommandBody::Noop => {
    //                 NoopHandler::handle(&mut imap_sock, &cmd).await
    //             }
    //             CommandBody::Capability => {
    //                 CapabilityHandler::handle(&mut imap_sock, &cmd).await
    //             }
    //             CommandBody::Login { username, password } => {
    //                 // debug!("LOGIN");
    //                 // debug!(
    //                 //     "username: {:?}, password: {:?}",
    //                 //     username, password
    //                 // );
    //                 // imap_sock.ok_completed(&cmd.tag, "LOGIN").await;
    //                 LoginHandler::handle(&mut imap_sock, &cmd).await
    //             }
    //             CommandBody::List {
    //                 reference: _,
    //                 mailbox_wildcard: _,
    //             } => ListHandler::handle(&mut imap_sock, &cmd, reference, mailbox_wildcard).await,
    //             // CommandBody::List {
    //             //     reference,
    //             //     mailbox_wildcard,
    //             // } => {
    //             //     debug!("LIST");
    //             //     debug!(
    //             //         "reference: {:?}, mailbox: {:?}",
    //             //         reference, mailbox_wildcard
    //             //     );
    //             //     imap_sock
    //             //         .status(r###"LIST (\Noselect) "/" ~/Mail/foo"###)
    //             //         .await;
    //             //     imap_sock
    //             //         .status(r###"LIST () "/" ~/Mail/meetings"###)
    //             //         .await;
    //             //     imap_sock.ok_completed(&cmd.tag, "LIST").await;
    //             // }
    //             // CommandBody::Select { mailbox } => {
    //             //     debug!("SELECT");
    //             //     debug!("mailbox: {:?}", mailbox);
    //             //     imap_sock.status("23 EXISTS").await;
    //             //     imap_sock.status("23 RECENT").await;
    //             //     imap_sock
    //             //         .status("OK [UNSEEN 12] Message 12 is first unseen")
    //             //         .await;
    //             //     imap_sock
    //             //         .status("OK [UIDVALIDITY 3857529045] UIDs valid")
    //             //         .await;
    //             //     imap_sock
    //             //         .status("OK [UIDNEXT 4392] Predicted next UID")
    //             //         .await;
    //             //     imap_sock.status("OK [PERMANENTFLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)] Limited").await;
    //             //     imap_sock
    //             //         .ok_completed2(cmd.tag.as_ref(), "[READ-WRITE] SELECT")
    //             //         .await;
    //             // }
    //             // CommandBody::Examine { mailbox } => {
    //             //     debug!("EXAMINE");
    //             //     debug!("mailbox: {:?}", mailbox);
    //             //     imap_sock.status("23 EXISTS").await;
    //             //     imap_sock.status("23 RECENT").await;
    //             //     imap_sock
    //             //         .status("OK [UNSEEN 12] Message 12 is first unseen")
    //             //         .await;
    //             //     imap_sock
    //             //         .status("OK [UIDVALIDITY 3857529045] UIDs valid")
    //             //         .await;
    //             //     imap_sock
    //             //         .status("OK [UIDNEXT 4392] Predicted next UID")
    //             //         .await;
    //             //     imap_sock.status("OK [PERMANENTFLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)] Limited").await;
    //             //     imap_sock
    //             //         .ok_completed2(cmd.tag.as_ref(), "[READ-ONLY] EXAMINE")
    //             //         .await;
    //             // }
    //             // CommandBody::Search {
    //             //     charset,
    //             //     criteria,
    //             //     uid,
    //             // } => {
    //             //     debug!("SEARCH");
    //             //     debug!(
    //             //         "charset: {:?}, criteria: {:?}, uid: {:?}",
    //             //         charset, criteria, uid
    //             //     );
    //             //     imap_sock.status("SEARCH 1").await;
    //             //     imap_sock.ok_completed(&cmd.tag, "SEARCH").await;
    //             // }
    //             // CommandBody::Close => {
    //             //     imap_sock.ok_completed(&cmd.tag, "CLOSE").await;
    //             //     return Ok(CommandPipe::Quit);
    //             // }
    //             // CommandBody::Check => {
    //             //     debug!("CHECK");
    //             //     imap_sock.ok_completed(&cmd.tag, "CHECK").await;
    //             // }
    //             _ => {
    //                 debug!("Other");
    //                 Ok(CommandPipe::Next(cmd, None))
    //             }
    //         }

    //         // Ok(CommandPipe::Next(cmd, None))
    //     }
    //     Err(e) => Err(e.into()),
    // }
}

pub fn command_decode(buf: &[u8]) -> Result<Command> {
    let (_remainder, parsed) = Command::decode(buf)?;
    Ok(parsed)
}
