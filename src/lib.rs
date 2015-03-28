#![allow(dead_code)]

extern crate hyper;
extern crate rustc_serialize;

pub use client::Client;
pub use schema::{Machine, Unit, UnitOption, UnitState, UnitStates};

mod api;
mod client;
mod schema;
mod serialize;
