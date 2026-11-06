//! The `AUTH` command used to support TLS
//!
//! A client requests TLS with the AUTH command and then decides if it
//! wishes to secure the data connections by use of the PBSZ and PROT
//! commands.

use crate::{
    auth::UserDetail,
    server::controlchan::{
        Reply, ReplyCode,
        error::ControlChanError,
        handler::{CommandContext, CommandHandler},
    },
    storage::{Metadata, StorageBackend},
};
use async_trait::async_trait;

// The parameter that can be given to the `AUTH` command.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AuthParam {
    Ssl,
    Tls,
}

#[derive(Debug)]
pub struct Auth {
    protocol: AuthParam,
}

impl Auth {
    pub fn new(protocol: AuthParam) -> Self {
        Auth { protocol }
    }
}

#[async_trait]
impl<Storage, User> CommandHandler<Storage, User> for Auth
where
    User: UserDetail + 'static,
    Storage: StorageBackend<User> + 'static,
    Storage::Metadata: Metadata,
{
    #[tracing_attributes::instrument]
    async fn handle(&self, args: CommandContext<Storage, User>) -> Result<Reply, ControlChanError> {
        match (args.tls_configured, self.protocol.clone()) {
            // The control loop starts TLS as soon as this reply is sent: the client begins the
            // handshake right after reading it (RFC 4217, section 4), so no further line may be
            // read from the plain connection in between.
            (true, AuthParam::Tls) => Ok(Reply::new(ReplyCode::AuthOkayNoDataNeeded, "Upgrading to TLS")),
            (true, AuthParam::Ssl) => Ok(Reply::new(ReplyCode::CommandNotImplementedForParameter, "Auth SSL not implemented")),
            (false, _) => Ok(Reply::new(ReplyCode::CommandNotImplemented, "TLS/SSL not configured")),
        }
    }
}
