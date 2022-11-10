use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};
use tonic::{transport::Server, Request, Response, Status};
mod hello;

// defining a struct for service
#[derive(Default)]
pub struct MySay {}

// implementing rpc for service defined in .proto
