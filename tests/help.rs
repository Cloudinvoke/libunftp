#![allow(missing_docs)]

use libunftp::ServerBuilder;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use unftp_sbe_fs::Filesystem;

const DEFAULT_ADDR: &str = "127.0.0.1:2170";
const CUSTOM_ADDR: &str = "127.0.0.1:2171";

async fn serve(addr: &'static str, help_text: Option<Vec<&'static str>>) {
    let root = std::env::temp_dir();
    let mut builder = ServerBuilder::new(Box::new(move || Filesystem::new(root.clone()).unwrap())).greeting("Welcome test");
    if let Some(lines) = help_text {
        builder = builder.help_text(lines);
    }
    builder.build().unwrap().listen(addr).await.unwrap();
}

/// The reply to HELP, every line of it, once logged in
async fn help_reply(addr: &str) -> Vec<String> {
    let mut stream = None;
    for _ in 0..50 {
        if let Ok(s) = TcpStream::connect(addr).await {
            stream = Some(s);
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let mut stream = BufReader::new(stream.expect("server did not start"));
    let mut line = String::new();

    async fn send(stream: &mut BufReader<TcpStream>, line: &mut String, command: &str, expected: &str) {
        stream.get_mut().write_all(command.as_bytes()).await.unwrap();
        line.clear();
        stream.read_line(line).await.unwrap();
        assert!(line.starts_with(expected), "{command:?}: {line:?}");
    }
    stream.read_line(&mut line).await.unwrap();
    assert_eq!(line, "220 Welcome test\r\n");
    send(&mut stream, &mut line, "USER anonymous\r\n", "331").await;
    send(&mut stream, &mut line, "PASS anonymous\r\n", "230").await;

    stream.get_mut().write_all(b"HELP\r\n").await.unwrap();
    let mut reply = Vec::new();
    loop {
        line.clear();
        stream.read_line(&mut line).await.unwrap();
        reply.push(line.trim_end_matches("\r\n").to_string());
        // The last line has the code followed by a space
        if line.starts_with("214 ") {
            return reply;
        }
        assert!(reply.len() < 20, "no end to the reply: {reply:?}");
    }
}

#[tokio::test(flavor = "current_thread")]
async fn help_text_default_is_unchanged() {
    tokio::spawn(serve(DEFAULT_ADDR, None));
    let reply = help_reply(DEFAULT_ADDR).await;
    assert_eq!(reply[0], "214-Help:");
    assert!(reply.iter().any(|l| l.starts_with("Powered by libunftp: ")), "{reply:?}");
}

#[tokio::test(flavor = "current_thread")]
async fn help_text_replaces_the_reply() {
    tokio::spawn(serve(
        CUSTOM_ADDR,
        Some(vec!["The following commands are recognized.", " ABOR CWD LIST", "Help OK."]),
    ));
    let reply = help_reply(CUSTOM_ADDR).await;
    assert_eq!(reply, ["214-The following commands are recognized.", " ABOR CWD LIST", "214 Help OK."]);
}

#[test]
fn help_text_rejects_embedded_crlf() {
    // An embedded reply terminator could end the 214 response early and leave
    // forged reply lines for the next command.
    let root = std::env::temp_dir();
    let builder = ServerBuilder::new(Box::new(move || Filesystem::new(root.clone()).unwrap())).help_text(["Start\r\n214 End\r\n220 Forged", "Last"]);

    assert!(builder.build().is_err(), "HELP text must contain one protocol line per item");
}
