#![allow(missing_docs)]

pub mod common;

use common::{read_from_server, send_to_server, tcp_connect};

#[tokio::test(flavor = "current_thread")]
async fn session_survives_invalid_parameter() {
    common::initialize().await;

    let stream = tcp_connect().await.unwrap();
    let mut buffer = vec![0_u8; 1024];
    assert_eq!(read_from_server(&mut buffer, &stream).await, "220 Welcome test\r\n");

    send_to_server("REST abc\r\n", &stream).await;
    assert_eq!(read_from_server(&mut buffer, &stream).await, "501 Invalid Parameter\r\n");

    send_to_server("NOOP\r\n", &stream).await;
    assert_eq!(read_from_server(&mut buffer, &stream).await, "200 Successfully did nothing\r\n");

    common::finalize().await;
}
