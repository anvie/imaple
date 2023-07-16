use crate::imap_serv::*;
use crate::result::Result;

use imap_codec::search::SearchKey;
use imap_codec::{
    command::Command,
    core::*,
    mailbox::{ListMailbox, Mailbox},
    secret::Secret,
};

use log::debug;

use tokio::io::{AsyncRead, AsyncWrite};

macro_rules! command_handler {
    ($name:ident, $cmd:ident, ($imap_sock:ident, $cmd2:ident ) => $cmd_body:expr ) => {
        pub(crate) struct $name;
        impl $name {
            pub async fn handle<'b, 'c, IO>(
                $imap_sock: &mut IMAPServ<'b, IO>,
                $cmd2: &Command<'c>,
            ) -> Result<CommandPipe<'c>>
            where
                IO: AsyncRead + AsyncWrite + Unpin,
            {
                $cmd_body
            }
        }
    };

    ($name:ident, $cmd:ident, ($imap_sock:ident, $cmd2:ident,  [$( $arg:ident : $tpe:ty ),*]  ) => $cmd_body:expr ) => {
        pub(crate) struct $name;
        impl $name {
            pub async fn handle<'b, 'c, IO>(
                $imap_sock: &mut IMAPServ<'b, IO>,
                $cmd2: &Command<'c>,
                $( $arg : $tpe, )*
            ) -> Result<CommandPipe<'c>>
            where
                IO: AsyncRead + AsyncWrite + Unpin,
            {
                $cmd_body
            }
        }
    };
}

command_handler!(NoopHandler, Noop, (s, cmd) => {
    s.status("23 EXISTS").await;
    s.ok_completed(&cmd.tag, "NOOP").await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});

command_handler!(CapabilityHandler, Capability, (s, cmd) => {
    s.status("CAPABILITY IMAP4rev1 STARTTLS").await;
    s.ok_completed(&cmd.tag, cmd.name()).await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});

command_handler!(LoginHandler, Login, (s, cmd, [ username: AString<'_>, password: Secret<AString<'_>> ]) => {
    debug!("username: {:?}, password: {:?}", username, password);
    s.ok_completed(&cmd.tag, "LOGIN").await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});

command_handler!(ListHandler, List, (s, cmd,
    [reference:Mailbox<'_>, mailbox_wildcard: ListMailbox<'_>] ) => {

    debug!(
        "reference: {:?}, mailbox: {:?}",
        reference, mailbox_wildcard
    );

    s.status(r###"LIST (\Noselect) "/" ~/Mail/foo"###)
        .await;
    s.status(r###"LIST () "/" ~/Mail/meetings"###)
        .await;
    s.ok_completed(&cmd.tag, "LIST").await;

    Ok(CommandPipe::Next(cmd.clone(), None))
});

command_handler!(SelectHandler, Select, (s, cmd, [ mailbox: Mailbox<'_> ]) => {
    debug!("mailbox: {:?}", mailbox);
    s.status("23 EXISTS").await;
    s.status("23 RECENT").await;
    s.status("OK [UNSEEN 12] Message 12 is first unseen")
        .await;
    s.status("OK [UIDVALIDITY 3857529045] UIDs valid")
        .await;
    s.status("OK [UIDNEXT 4392] Predicted next UID")
        .await;
    s.status("OK [PERMANENTFLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)] Limited").await;
    s.ok_completed2(cmd.tag.as_ref(), "[READ-WRITE] SELECT")
        .await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});

command_handler!(SearchHandler, Search, (s, cmd, [ charset: Option<Charset<'_>>,  criteria: SearchKey<'_>, uid:bool ]) => {
    debug!(
        "charset: {:?}, criteria: {:?}, uid: {:?}",
        charset, criteria, uid
    );
    s.status("SEARCH 3 5").await;
    s.ok_completed(&cmd.tag, "SEARCH").await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});

command_handler!(LogoutHandler, Logout, (s, cmd) => {
    s.ok_completed(&cmd.tag, "LOGOUT").await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});
