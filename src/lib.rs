#![allow(dead_code)]

extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub use client::Client;
pub use error::FleetError;
pub use schema::{Machine, Unit, UnitOption, UnitState, UnitStates};

mod client;
mod error;
mod schema;
mod serialize;
