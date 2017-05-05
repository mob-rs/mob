// #![deny(warnings)]
#![allow(unmounted_route)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod error;
pub mod db;
pub mod models;
pub mod schema;
pub mod web;

type Result<T> = std::result::Result<T, error::Error>;
