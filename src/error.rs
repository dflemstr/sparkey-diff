use std::io;
use std::sync;

use protobuf;
use sparkey;

error_chain! {
    links {
        Sparkey(sparkey::error::Error, sparkey::error::ErrorKind);
    }
    foreign_links {
        IO(io::Error);
        Protobuf(protobuf::ProtobufError);
        Recv(sync::mpsc::RecvError);
    }
}
