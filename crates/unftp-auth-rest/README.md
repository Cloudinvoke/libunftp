# unftp-auth-rest

[![Crate Version](https://img.shields.io/crates/v/unftp-auth-rest.svg)](https://crates.io/crates/unftp-auth-rest)
[![API Docs](https://docs.rs/unftp-auth-rest/badge.svg)](https://docs.rs/unftp-auth-rest)
[![Crate License](https://img.shields.io/crates/l/unftp-auth-rest.svg)](https://crates.io/crates/unftp-auth-rest)
[![Follow on Telegram](https://img.shields.io/badge/Follow%20on-Telegram-brightgreen.svg)](https://t.me/unftp)

An authenticator backend for [libunftp](https://github.com/Cloudinvoke/libunftp) that calls an
HTTP API to authenticate FTP users.

## Usage

Configure a [`Builder`](https://docs.rs/unftp-auth-rest/latest/unftp_auth_rest/struct.Builder.html)
with the request URL and method. Username, password, and source IP placeholders can be used in
the URL or request body. Set a JSON Pointer with `with_selector` to choose a value from the
response, then set `with_regex` to match the accepted value. For example, a response of
`{"status":"success"}` has the selector `/status`.

Register the built authenticator with [`ServerBuilder::authenticator`](https://docs.rs/libunftp/latest/libunftp/struct.ServerBuilder.html#method.authenticator).
See the [complete example](https://github.com/Cloudinvoke/libunftp/blob/master/crates/unftp-auth-rest/examples/rest.rs)
and the [crate documentation](https://docs.rs/unftp-auth-rest/latest/unftp_auth_rest/) for the
builder configuration.

## License

You're free to use, modify and distribute this software under the terms of the [Apache License v2.0](http://www.apache.org/licenses/LICENSE-2.0).
