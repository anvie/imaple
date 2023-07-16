use log::debug;
use rustls::{Certificate, PrivateKey};
use std::fs::File;
use std::io::BufReader;

pub(crate) fn load_certificates_from_pem(
    path: &str,
) -> std::io::Result<Vec<Certificate>> {
    debug!("loading cert from `{}`", path);

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader)?;

    Ok(certs.into_iter().map(Certificate).collect())
}

pub(crate) fn load_private_key_from_file_pkcs(
    path: &str,
) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;

    match keys.len() {
        0 => {
            Err(format!("No PKCS8-encoded private key found in {path}").into())
        }
        1 => Ok(PrivateKey(keys.remove(0))),
        _ => Err(format!(
            "More than one PKCS8-encoded private key found in {path}"
        )
        .into()),
    }
}

pub(crate) fn load_private_key_from_file(
    path: &str,
) -> Result<PrivateKey, Box<dyn std::error::Error>> {
    debug!("Loading private key from `{}`", path);

    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut keys = rustls_pemfile::rsa_private_keys(&mut reader)?;

    match keys.len() {
        0 => {
            Err(format!("No PKCS8-encoded private key found in {path}").into())
        }
        1 => Ok(PrivateKey(keys.remove(0))),
        _ => Err(format!(
            "More than one PKCS8-encoded private key found in {path}"
        )
        .into()),
    }
}
