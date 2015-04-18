//! Crate fleet provides a client for the [fleet](https://github.com/coreos/fleet) API.
//!
//! All of the public types are rexported and available directly from the crate root. `Client` is
//! the entry point for all API calls.
extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub use client::Client;
pub use error::FleetError;
pub use schema::{
    Machine,
    MachinePage,
    Unit,
    UnitOption,
    UnitPage,
    UnitState,
    UnitStatePage,
    UnitStates
};

mod client;
mod error;
mod schema;
mod serialize;
