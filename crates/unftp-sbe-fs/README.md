# unftp-sbe-fs

[![Crate Version](https://img.shields.io/crates/v/unftp-sbe-fs.svg)](https://crates.io/crates/unftp-sbe-fs)
[![API Docs](https://docs.rs/unftp-sbe-fs/badge.svg)](https://docs.rs/unftp-sbe-fs)
[![Crate License](https://img.shields.io/crates/l/unftp-sbe-fs.svg)](https://crates.io/crates/unftp-sbe-fs)
[![Follow on Telegram](https://img.shields.io/badge/Follow%20on-Telegram-brightgreen.svg)](https://t.me/unftp)

`unftp-sbe-fs` stores files on the local filesystem for a
[libunftp](https://github.com/Cloudinvoke/libunftp) FTP server. It confines
storage operations to the configured root directory.

## Getting started

If you've got Rust and cargo installed, create your project with

```sh
cargo new myftp
```

Add libunftp, this backend, and Tokio to your project's dependencies in `Cargo.toml`.

```toml
[dependencies]
libunftp = "0.25.0"
unftp-sbe-fs = "0.4.1"
tokio = { version = "1", features = ["full"] }
```

Now you're ready to develop your server!
Add the following to `src/main.rs`:

```rust
use libunftp::ServerBuilder;
use unftp_sbe_fs::Filesystem;

#[tokio::main]
pub async fn main() {
    let ftp_home = std::env::temp_dir();
    let server = ServerBuilder::new(Box::new(move || Filesystem::new(ftp_home.clone()).unwrap()))
        .greeting("Welcome to my FTP server")
        .passive_ports(50000..=65535)
        .build()
        .unwrap();

    server.listen("127.0.0.1:2121").await.unwrap();
}
```

You can now run your server with `cargo run` and connect to `localhost:2121` with your favourite FTP client e.g.:

```sh
lftp -p 2121 localhost
```

For more help refer to:

- the [examples](https://github.com/Cloudinvoke/libunftp/tree/master/crates/unftp-sbe-fs/examples) directory.
- the [API Documentation](https://docs.rs/unftp-sbe-fs/latest/unftp_sbe_fs/).
- [unFTP server](https://github.com/Cloudinvoke/unFTP), a server built on top of libunftp.

## Getting help and staying informed

Support is given on a best effort basis. You are welcome to engage us
on [the discussions page](https://github.com/Cloudinvoke/libunftp/discussions)
or create a Github issue.

You can also follow news and talk to us on [Telegram](https://t.me/unftp)

## Contributing

Thank you for your interest in contributing to unftp-sbe-fs!

Please feel free to create a Github issue if you encounter any problems.

Want to submit a feature request or develop your own storage or authentication back-end? Then head over to
our [contribution guide](https://github.com/Cloudinvoke/libunftp/blob/master/CONTRIBUTING.md).

## License

You're free to use, modify and distribute this software under the terms of
the [Apache License v2.0](http://www.apache.org/licenses/LICENSE-2.0).
