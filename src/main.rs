// Copyright (C) 2023 Neuversity
// All Rights Reserved.
//
// NOTICE: All information contained herein is, and remains
// the property of Neuversity.
// The intellectual and technical concepts contained
// herein are proprietary to Neuversity
// and are protected by trade secret or copyright law.
// Dissemination of this information or reproduction of this material
// is strictly forbidden unless prior written permission is obtained
// from Neuversity.
#![allow(dead_code)]

use clap::Parser;

use dotenvy::dotenv;
use tokio_rustls::TlsAcceptor;

use log::debug;
use result::Result;
use rustls::ServerConfig;
use serde::Deserialize;

use std::sync::Arc;
use std::{env, fs, io::ErrorKind, process::exit};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;

mod cert;
mod error;
mod handlers;
mod imap;
mod imap_serv;
mod result;

use imap::{process_command, CommandPipe, IMAPServ};

use crate::cert::{load_certificates_from_pem, load_private_key_from_file};

#[derive(Parser, Debug)]
#[command(name = "nu-id-smtp")]
#[command(about = "Simple SMTP mail wrapper/proxy")]
#[command(author, version, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "default.conf")]
    config: String,
}
#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_imap_port")]
    imap_port: u16,

    #[serde(default = "default_smtp_port")]
    smtp_port: u16,
}

fn default_imap_port() -> u16 {
    143
}

fn default_smtp_port() -> u16 {
    25
}

// #[async_std::main]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    env_logger::init();

    let args = Args::parse();

    let config: Config = match fs::read_to_string(&args.config) {
        Ok(config) => toml::from_str(&config).unwrap(),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("`{}` not exists.", args.config);
                exit(2);
            } else {
                panic!("Error: {}", e);
            }
        }
    };

    if let Err(err) = start_imap_server(config).await {
        eprintln!("Error: {}", err);
        exit(3);
    }

    Ok(())
}

async fn start_imap_server(
    conf: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{}", conf.imap_port);

    let listener = TcpListener::bind(addr).await?;

    let cert_chain = load_certificates_from_pem(
        &env::var("CAFILE").expect("No CAFILE env var"),
    )
    .expect("No cert file provided");

    let mut keys = vec![load_private_key_from_file(
        &env::var("KEYFILE").expect("No KEYFILE env var"),
    )
    .expect("No private key provided")];

    // let mut keys = rsa_private_keys(key_file).unwrap();
    // config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0))
        .unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(config));

    println!("Starting IMAP server at port {}...", conf.imap_port);

    loop {
        let (socket, _) = listener.accept().await?;

        debug!("socket: {:?}", socket);

        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            let mut socket = match acceptor.accept(socket).await {
                Ok(socket) => socket,
                Err(e) => {
                    eprintln!("Failed to accept socket; err = {:?}", e);
                    return;
                }
            };

            let _ = socket.write_all(b"* OK IMAP4rev1 server ready\r\n").await;

            loop {
                let mut n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // apabila buff tidak diakhiri dengan CRLF maka tambahkan CRLF di akhir array
                if &buf[n - 2..n] != &[13, 10] {
                    buf[n - 1] = 13;
                    buf[n] = 10;
                    n += 1;
                }

                debug!("COMMAND: {:?}", String::from_utf8_lossy(&buf[0..n]));

                let cmd_pipe = match process_command(&buf[0..n], &mut socket)
                    .await
                {
                    Ok(cmd_pipe) => cmd_pipe,
                    Err(e) => {
                        eprintln!("Failed to decode command; err = {:?}", e);
                        return;
                    }
                };

                let _ = process_command_result(&cmd_pipe, &mut socket);

                match cmd_pipe {
                    CommandPipe::Quit => return,
                    _ => {}
                }
            }
        });
    }
}

fn process_command_result<'a, IO>(
    cmd: &CommandPipe<'a>,
    socket: &mut IO,
) -> Result<()>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    let _imap_serv = IMAPServ::new(socket);
    match cmd {
        CommandPipe::Next(_cmd, _prev) => {
            // debug!("Next: {:?}", cmd);
        }
        // CommandPipe::Quit(tag) => {
        //     debug!("Quit");
        //     // let resp = format!("{tag} OK CLOSE completed\r\n", tag=tag.as_ref());
        //     // let _ = socket.write_all(resp.as_bytes()).await;
        //     imap_serv.ok_completed(tag.as_ref(), "CLOSE").await;
        // }
        _ => {}
    }

    Ok(())
}
