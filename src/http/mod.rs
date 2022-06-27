// export { Server, Method, ParseError } from './server';
pub use request::{Method, ParseError, Request};
pub use server::Server;

// import * as server from './server';
mod server;

// export * as request from './request';
pub mod request;
pub mod query_string;
