# unftp-auth-jsonfile

[![Crate Version](https://img.shields.io/crates/v/unftp-auth-jsonfile.svg)](https://crates.io/crates/unftp-auth-jsonfile)
[![API Docs](https://docs.rs/unftp-auth-jsonfile/badge.svg)](https://docs.rs/unftp-auth-jsonfile)
[![Crate License](https://img.shields.io/crates/l/unftp-auth-jsonfile.svg)](https://crates.io/crates/unftp-auth-jsonfile)
[![Follow on Telegram](https://img.shields.io/badge/Follow%20on-Telegram-brightgreen.svg)](https://t.me/unftp)

An authenticator back-end for [libunftp](https://github.com/Cloudinvoke/libunftp) that authenticates against a JSON file.

## Usage

Create a JSON file containing an array of users:

```json
[
  { "username": "alice", "password": "example-password" }
]
```

Load it with [`JsonFileAuthenticator::from_file`](https://docs.rs/unftp-auth-jsonfile/latest/unftp_auth_jsonfile/struct.JsonFileAuthenticator.html#method.from_file)
and pass the authenticator to [`ServerBuilder::authenticator`](https://docs.rs/libunftp/latest/libunftp/struct.ServerBuilder.html#method.authenticator).
The backend also accepts PBKDF2 credentials, optional IP allow lists, and client certificate rules;
see the [crate documentation](https://docs.rs/unftp-auth-jsonfile/latest/unftp_auth_jsonfile/)
for their JSON formats. Use [`from_json`](https://docs.rs/unftp-auth-jsonfile/latest/unftp_auth_jsonfile/struct.JsonFileAuthenticator.html#method.from_json)
when the credentials come from another source.

For a complete server, see the [JSON-file authentication example](https://github.com/Cloudinvoke/libunftp/blob/master/crates/unftp-auth-jsonfile/examples/jsonfile_auth.rs).

## License

You're free to use, modify and distribute this software under the terms of the [Apache License v2.0](http://www.apache.org/licenses/LICENSE-2.0).
