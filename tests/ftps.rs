#![allow(missing_docs)]

//! FTPS control channel upgrade via `AUTH TLS` (RFC 4217, section 4).

use libunftp::ServerBuilder;
use rustls::pki_types::{CertificateDer, ServerName, pem::PemObject};
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;
use unftp_sbe_fs::Filesystem;

const CERTS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources/ftps/server.pem");
const KEY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources/ftps/server.key");
const CA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources/ftps/ca.pem");

/// How many control connections each test upgrades.
const SESSIONS: usize = 50;

async fn start_server(addr: &'static str) {
    let root = std::env::temp_dir();
    let server = ServerBuilder::new(Box::new(move || Filesystem::new(root.clone()).unwrap()))
        .greeting("Welcome test")
        .ftps(CERTS, KEY)
        .build()
        .unwrap();
    tokio::spawn(server.listen(addr));
    for _ in 0..50 {
        if tokio::net::TcpStream::connect(addr).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("the FTPS test server did not start on {addr}");
}

fn client_config() -> Arc<ClientConfig> {
    let mut roots = RootCertStore::empty();
    for cert in CertificateDer::pem_file_iter(CA).unwrap() {
        roots.add(cert.unwrap()).unwrap();
    }
    Arc::new(ClientConfig::builder().with_root_certificates(roots).with_no_client_auth())
}

/// Reads one reply line, one byte at a time so that nothing after the line is consumed.
fn read_line(stream: &mut impl Read) -> std::io::Result<String> {
    let mut line = Vec::new();
    let mut byte = [0_u8; 1];
    while !line.ends_with(b"\r\n") {
        if stream.read(&mut byte)? == 0 {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        line.push(byte[0]);
    }
    Ok(String::from_utf8_lossy(&line).into_owned())
}

/// Upgrades one control connection with `AUTH TLS` and sends a command over TLS.
///
/// The client sends its TLS ClientHello as soon as it has read the 234 reply, or, when
/// `pipelined` is set, in the same write as the `AUTH TLS` command.
fn upgrade_session(addr: &str, config: &Arc<ClientConfig>, pipelined: bool) -> Result<(), String> {
    let mut stream = TcpStream::connect(addr).map_err(|e| e.to_string())?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).map_err(|e| e.to_string())?;
    let greeting = read_line(&mut stream).map_err(|e| format!("greeting: {e}"))?;
    if !greeting.starts_with("220") {
        return Err(format!("greeting: {greeting:?}"));
    }

    let mut tls = ClientConnection::new(config.clone(), ServerName::try_from("localhost").unwrap()).map_err(|e| e.to_string())?;
    let mut client_hello = Vec::new();
    tls.write_tls(&mut client_hello).map_err(|e| e.to_string())?;

    let reply = if pipelined {
        stream
            .write_all(&[b"AUTH TLS\r\n".as_slice(), &client_hello].concat())
            .map_err(|e| e.to_string())?;
        read_line(&mut stream).map_err(|e| format!("AUTH reply: {e}"))?
    } else {
        stream.write_all(b"AUTH TLS\r\n").map_err(|e| e.to_string())?;
        let reply = read_line(&mut stream).map_err(|e| format!("AUTH reply: {e}"))?;
        stream.write_all(&client_hello).map_err(|e| e.to_string())?;
        reply
    };
    if !reply.starts_with("234") {
        return Err(format!("AUTH reply: {reply:?}"));
    }

    let mut channel = rustls::Stream::new(&mut tls, &mut stream);
    channel.write_all(b"NOOP\r\n").map_err(|e| format!("TLS handshake: {e}"))?;
    let reply = read_line(&mut channel).map_err(|e| format!("NOOP over TLS: {e}"))?;
    if !reply.starts_with("200") {
        return Err(format!("NOOP over TLS: {reply:?}"));
    }
    Ok(())
}

async fn upgrade_sessions(addr: &'static str, pipelined: bool) {
    start_server(addr).await;
    let failures = tokio::task::spawn_blocking(move || {
        let config = client_config();
        (0..SESSIONS)
            .filter_map(|_| upgrade_session(addr, &config, pipelined).err())
            .collect::<Vec<_>>()
    })
    .await
    .unwrap();
    assert!(
        failures.is_empty(),
        "{} of {SESSIONS} upgrades failed, first: {}",
        failures.len(),
        failures.first().unwrap()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn auth_tls_upgrades_when_the_client_hello_follows_the_reply() {
    upgrade_sessions("127.0.0.1:2160", false).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn auth_tls_upgrades_when_the_client_hello_arrives_with_the_command() {
    upgrade_sessions("127.0.0.1:2161", true).await;
}
