# unftp-auth-pam

[![Crate Version](https://img.shields.io/crates/v/unftp-auth-pam.svg)](https://crates.io/crates/unftp-auth-pam)
[![API Docs](https://docs.rs/unftp-auth-pam/badge.svg)](https://docs.rs/unftp-auth-pam)
[![Crate License](https://img.shields.io/crates/l/unftp-auth-pam.svg)](https://crates.io/crates/unftp-auth-pam)
[![Follow on Telegram](https://img.shields.io/badge/Follow%20on-Telegram-brightgreen.svg)](https://t.me/unftp)

An authenticator backend for [libunftp](https://github.com/Cloudinvoke/libunftp) that authenticates
against [PAM](https://en.wikipedia.org/wiki/Pluggable_authentication_module).

## Usage

This crate is for Unix systems. Install the PAM development libraries when building it, and
configure a PAM service on the server host. Pass that service name to
[`PamAuthenticator::new`](https://docs.rs/unftp-auth-pam/latest/unftp_auth_pam/struct.PamAuthenticator.html#method.new),
then register the authenticator with [`ServerBuilder::authenticator`](https://docs.rs/libunftp/latest/libunftp/struct.ServerBuilder.html#method.authenticator).

```rust
use unftp_auth_pam::PamAuthenticator;

let authenticator = PamAuthenticator::new("ftp");
```

The service name must match the host's PAM configuration. The authenticator uses the password
supplied by the FTP client.

## License

You're free to use, modify and distribute this software under the terms of the [Apache License v2.0](http://www.apache.org/licenses/LICENSE-2.0).
