//! An abstraction over [fetching][fetch()] a pack from the server.
//!
//! This implementation hides the transport layer, statefulness and the protocol version to the [fetch delegate][fetch::Delegate],
//! the actual client implementation.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

/// A convenience export allowing users of git-protocol to use the transport layer without their own cargo dependency.
pub use git_transport;

mod remote_progress;
pub use remote_progress::RemoteProgress;

///
pub mod credentials;
///
pub mod fetch;

#[doc(inline)]
pub use fetch::fetch;
