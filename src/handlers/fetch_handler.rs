use std::convert::TryFrom;
use std::num::NonZeroU32;

use crate::imap_serv::IMAPServ;
use crate::result::Result;

use imap_codec::codec::Encode;
use imap_codec::envelope::{Address, Envelope};
use imap_codec::fetch::{
    Macro, MacroOrMessageDataItemNames, MessageDataItem, MessageDataItemName,
};
use imap_codec::response::Data;

use imap_codec::core::*;
use log::debug;
use tokio::io::{AsyncRead, AsyncWrite};

pub async fn handle_seq_value<IO>(
    s: &mut IMAPServ<'_, IO>,
    seq_value: u32,
    macro_or_item_names: MacroOrMessageDataItemNames<'_>,
) -> Result<()>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    let mut items = vec![MessageDataItem::Rfc822Size(1337)];

    match macro_or_item_names {
        MacroOrMessageDataItemNames::Macro(macro_name) => {
            debug!("macro_name: {:?}", macro_name);
            match macro_name {
                Macro::All => {
                    items.push(build_envelope(seq_value));
                }
                Macro::Full => {
                    items.push(build_envelope(seq_value));
                }
                Macro::Fast => {
                    // @TODO(robin): code here
                }
            }
        }
        MacroOrMessageDataItemNames::MessageDataItemNames(
            msg_data_item_names,
        ) => {
            debug!("msg_data_item_names: {:?}", msg_data_item_names);
            for mi_name in msg_data_item_names.iter() {
                match mi_name {
                    MessageDataItemName::Envelope => {
                        items.push(build_envelope(seq_value));
                    }
                    MessageDataItemName::Body => todo!(),
                    MessageDataItemName::BodyExt {
                        section: _,
                        partial: _,
                        peek: _,
                    } => todo!(),
                    MessageDataItemName::BodyStructure => todo!(),
                    MessageDataItemName::Flags => todo!(),
                    MessageDataItemName::InternalDate => todo!(),
                    MessageDataItemName::Rfc822 => todo!(),
                    MessageDataItemName::Rfc822Header => todo!(),
                    MessageDataItemName::Rfc822Size => todo!(),
                    MessageDataItemName::Rfc822Text => todo!(),
                    MessageDataItemName::Uid => todo!(),
                }
            }
        }
    }

    let data = Data::Fetch {
        seq: NonZeroU32::new(seq_value).unwrap(),
        items: NonEmptyVec::try_from(items).unwrap(),
    };

    let resp = String::from_utf8(data.encode().dump()).unwrap();

    // let resp = format!("{data}\r\n", data=data);

    debug!(":> {}", resp);

    let _ = s.write_data(data).await;

    Ok(())
}

fn build_envelope<'a>(seq_value: u32) -> MessageDataItem<'a> {
    MessageDataItem::Envelope(Envelope {
        date: NString::try_from("Mon, 7 Feb 1994 21:52:25 -0800").unwrap(),
        subject: NString::try_from("Imaple is cool!").unwrap(),
        from: vec![Address {
            name: NString::try_from("Joe Q. Public").unwrap(),
            adl: NString(None),
            mailbox: NString::try_from("john.q.public").unwrap(),
            host: NString::try_from("example.com").unwrap(),
        }],
        sender: vec![Address {
            name: NString::try_from("Joe Q. Public").unwrap(),
            adl: NString(None),
            mailbox: NString::try_from("john.q.public").unwrap(),
            host: NString::try_from("example.com").unwrap(),
        }],
        reply_to: vec![],
        to: vec![Address {
            name: NString::try_from("Robin Syihab").unwrap(),
            adl: NString(None),
            mailbox: NString::try_from("robin").unwrap(),
            host: NString::try_from("nu.id").unwrap(),
        }],
        cc: vec![],
        bcc: vec![],
        in_reply_to: NString(None),
        message_id: NString::try_from(format!("{}", seq_value)).unwrap(),
    })
}
