// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/

extern crate protobuf;
extern crate protobuf_network_example;

use protobuf::*;
use protobuf::core::parse_length_delimited_from_reader;
use protobuf_network_example::proto::*;
use std::io;
#[allow(unused)]
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let request = try!(parse_length_delimited_from_reader::<Request>(&mut stream).or(Err(io::Error::new(io::ErrorKind::Other, "read error"))));
    println!("Got request: {}", request.get_field_in());
    let mut response = Response::new();
    response.set_out(String::from("hello client"));
    try!(response.write_length_delimited_to_writer(&mut stream).or(Err(io::Error::new(io::ErrorKind::Other, "write error"))));
    // Need this for everything to work.
    //try!(stream.shutdown(Shutdown::Write));
    Ok(())
}

fn run() -> io::Result<()> {
    let listener = try!(TcpListener::bind("127.0.0.1:12345"));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream).unwrap()
                });
            }
            Err(e) => { println!("Connection failed: {}", e); }
        }
    }
    drop(listener);
    Ok(())
}

fn main() {
    run().unwrap();
}

