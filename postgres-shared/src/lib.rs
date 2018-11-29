#![allow(unknown_lints)] // for clippy

extern crate fallible_iterator;
extern crate hex;
extern crate phf;
extern crate postgres_protocol;

pub mod error;
pub mod params;
pub mod rows;
pub mod stmt;
pub mod types;

/// Contains information necessary to cancel queries for a session.
#[derive(Copy, Clone, Debug)]
pub struct CancelData {
    /// The process ID of the session.
    pub process_id: i32,
    /// The secret key for the session.
    pub secret_key: i32,
}

/// An asynchronous notification.
#[derive(Clone, Debug)]
pub struct Notification {
    /// The process ID of the notifying backend process.
    pub process_id: i32,
    /// The name of the channel that the notify has been raised on.
    pub channel: String,
    /// The "payload" string passed from the notifying process.
    pub payload: String,
}
