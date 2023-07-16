use std::convert::TryFrom;
use std::num::NonZeroU32;

use crate::imap_serv::*;
use crate::result::Result;

use imap_codec::codec::Encode;
use imap_codec::envelope::{Address, Envelope};
use imap_codec::fetch::{MacroOrMessageDataItemNames, MessageDataItem};
use imap_codec::response::Data;
use imap_codec::search::SearchKey;
use imap_codec::sequence::{SeqOrUid, Sequence, SequenceSet};
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

command_handler!(SearchHandler, Search, (s, cmd,
    [ charset: Option<Charset<'_>>,  criteria: SearchKey<'_>, uid:bool ]) => {
    debug!(
        "charset: {:?}, criteria: {:?}, uid: {:?}",
        charset, criteria, uid
    );
    s.status("SEARCH 3 5").await;
    s.ok_completed(&cmd.tag, "SEARCH").await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});

command_handler!(LogoutHandler, Logout, (s, cmd) => {
    s.status("BYE IMAP4rev1 Server logging out").await;
    s.ok_completed(&cmd.tag, "LOGOUT").await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});

command_handler!(FetchHandler, Fetch, (s, cmd,
    [sequence_set: SequenceSet, _macro_or_item_names: MacroOrMessageDataItemNames<'_>, uid:bool] ) =>
{
    match sequence_set.0.as_ref()[0] {
        Sequence::Single(seq_or_uid) => {
            debug!("seq_or_uid: {:?}", seq_or_uid);
            match seq_or_uid {
                SeqOrUid::Value(seq) => {
                    let data = Data::Fetch {
                        seq: NonZeroU32::new(seq.get()).unwrap(),
                        items: NonEmptyVec::try_from(vec![
                            MessageDataItem::Rfc822Size(1337),
                            MessageDataItem::Envelope(
                                Envelope {
                                    date: NString::try_from("Mon, 7 Feb 1994 21:52:25 -0800").unwrap(),
                                    subject: NString::try_from("Imaple is cool!").unwrap(),
                                    from: vec![
                                        Address {
                                            name: NString::try_from("Joe Q. Public").unwrap(),
                                            adl: NString(None),
                                            mailbox: NString::try_from("john.q.public").unwrap(),
                                            host: NString::try_from("example.com").unwrap(),
                                        }
                                    ],
                                    sender: vec![
                                        Address {
                                            name: NString::try_from("Joe Q. Public").unwrap(),
                                            adl: NString(None),
                                            mailbox: NString::try_from("john.q.public").unwrap(),
                                            host: NString::try_from("example.com").unwrap(),
                                        }
                                    ],
                                    reply_to: vec![],
                                    to: vec![
                                        Address {
                                            name: NString::try_from("Robin Syihab").unwrap(),
                                            adl: NString(None),
                                            mailbox: NString::try_from("robin").unwrap(),
                                            host: NString::try_from("nu.id").unwrap(),
                                        }
                                    ],
                                    cc: vec![],
                                    bcc: vec![],
                                    in_reply_to: NString(None),
                                    message_id: NString::try_from(format!("{}", seq.get())).unwrap()
                                }
                            )
                        ]).unwrap()
                    };

                    let resp = String::from_utf8(data.encode().dump()).unwrap();

                    // let resp = format!("{data}\r\n", data=data);

                    debug!(":> {}", resp);

                    let _ = s.write_data(data).await;

                }
                SeqOrUid::Asterisk => {
                    debug!("uid: {:?}", uid);
                }
            }
        }
        Sequence::Range(a, b) => {
            debug!("seq ({:?}-{:?}):", a, b);
        }
    }

    s.ok_completed(&cmd.tag, cmd.name()).await;
    Ok(CommandPipe::Next(cmd.clone(), None))
});
