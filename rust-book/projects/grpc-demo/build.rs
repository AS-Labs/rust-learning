syntax = "proto3";

package hello;


service Say {
    rpc Send (SayRequest) returns (SayResponse);
}

message SayRequest {
    string name = 1;
}

message SayResponse {
    string message = 1;
}


tonic::include_proto!("hello");
