#![allow(dead_code)]
#![feature(core)]

extern crate hyper;
extern crate rustc_serialize;

pub use client::Client;
pub use error::FleetError;
pub use schema::{Machine, Unit, UnitOption, UnitState, UnitStates};

mod api;
mod client;
mod error;
mod schema;
mod serialize;
