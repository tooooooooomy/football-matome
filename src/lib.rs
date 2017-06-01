#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate toml;
extern crate chrono;
extern crate dotenv;
extern crate hyper;
extern crate mockito;
extern crate nickel;
extern crate rustc_serialize;

pub mod app;
pub mod config;
pub mod controllers;
pub mod models;
pub mod libraries;
pub mod schema;
pub mod services;
