// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/

extern crate protobuf;
extern crate protobuf_network_example;

use protobuf::*;
use protobuf::core::parse_length_delimited_from_reader;
use protobuf_network_example::proto::*;
use std::io;
#[allow(unused)]
use std::net::{Shutdown,TcpStream};

fn run() -> io::Result<()> {
    let mut stream = try!(TcpStream::connect("127.0.0.1:12345"));
    let mut request = Request::new();
    request.set_field_in(String::from("hello server"));
    try!(request.write_length_delimited_to_writer(&mut stream).or(Err(io::Error::new(io::ErrorKind::Other, "write error"))));
    // Need this for everything to work.
    //try!(stream.shutdown(Shutdown::Write));
    parse_length_delimited_from_reader::<Response>(&mut stream)
        .and_then(|r| {
            println!("Got response: {}", r.get_out());
            Ok(())
        })
        .or_else(|err| {
            println!("Error: {:?}", err);
            Err(io::Error::new(io::ErrorKind::Other, "read error"))
        })
}

fn main() {
    run().unwrap();
}
