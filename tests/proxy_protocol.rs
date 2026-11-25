#![cfg(feature = "proxy_protocol")]
#![allow(missing_docs)]

pub mod common;

use async_trait::async_trait;
use common::{read_from_server, send_to_server};
use lazy_static::lazy_static;
use libunftp::ServerBuilder;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use unftp_core::auth::{AuthenticationError, Authenticator, Credentials, Principal};
use unftp_sbe_fs::Filesystem;

const ADDR: &str = "127.0.0.1:2172";
const EXTERNAL_CONTROL_PORT: u16 = 2121;
const CLIENT_IP: &str = "203.0.113.7";

lazy_static! {
    static ref SEEN_SOURCE_IPS: Arc<Mutex<Vec<IpAddr>>> = Arc::new(Mutex::new(Vec::new()));
}

/// Accepts every login and records the source IP it was given.
#[derive(Debug)]
struct RecordingAuthenticator;

#[async_trait]
impl Authenticator for RecordingAuthenticator {
    async fn authenticate(&self, username: &str, creds: &Credentials) -> Result<Principal, AuthenticationError> {
        SEEN_SOURCE_IPS.lock().await.push(creds.source_ip);
        Ok(Principal {
            username: username.to_string(),
        })
    }
}

async fn run_in_proxy_protocol_mode() {
    let root = std::env::temp_dir();
    let server = ServerBuilder::new(Box::new(move || Filesystem::new(root.clone()).unwrap()))
        .authenticator(Arc::new(RecordingAuthenticator))
        .greeting("Welcome test")
        .proxy_protocol_mode(EXTERNAL_CONTROL_PORT)
        .build()
        .unwrap();
    server.listen(ADDR).await.unwrap();
}

async fn connect() -> TcpStream {
    for _ in 0..10 {
        if let Ok(stream) = TcpStream::connect(ADDR).await {
            return stream;
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    panic!("could not connect to {ADDR}");
}

#[tokio::test(flavor = "current_thread")]
async fn test_authenticator_receives_client_ip_from_proxy_header() {
    tokio::spawn(run_in_proxy_protocol_mode());

    let mut stream = connect().await;
    let header = format!("PROXY TCP4 {CLIENT_IP} 127.0.0.1 51234 {EXTERNAL_CONTROL_PORT}\r\n");
    stream.write_all(header.as_bytes()).await.unwrap();

    let mut buffer = vec![0_u8; 1024];
    assert_eq!(read_from_server(&mut buffer, &stream).await, "220 Welcome test\r\n");
    send_to_server("USER test\r\n", &stream).await;
    assert_eq!(read_from_server(&mut buffer, &stream).await, "331 Password Required\r\n");
    send_to_server("PASS test\r\n", &stream).await;
    assert_eq!(read_from_server(&mut buffer, &stream).await, "230 User logged in, proceed\r\n");

    let expected: IpAddr = CLIENT_IP.parse().unwrap();
    assert_eq!(*SEEN_SOURCE_IPS.lock().await, vec![expected]);
}
