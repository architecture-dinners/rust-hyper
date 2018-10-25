extern crate config;
extern crate http;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
#[allow(unused_imports)] // so we have them when we need them
#[macro_use(error, warn, info, debug, trace)]
extern crate slog_scope;
extern crate url;

mod cfg;
mod counter;
mod server;

pub use server::run;
