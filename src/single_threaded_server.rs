use std::io;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::Duration;

fn parse_incoming(stream: &mut TcpStream, buff: &mut [u8]) {
    println!("Incoming...");
    loop {
        let read = stream.read(buff);

        match read {
            Ok(size) => {
                print!("{}", String::from_utf8_lossy(&buff));
                io::stdout().flush().unwrap();
                buff.iter_mut().map(|x| *x = 0).count();
                println!("Read: {}", size);
            }
            Err(_) => {
                println!("Error while reading...");
            }
        }
    }
}

pub fn start_sts() {
    println!("Starting STS...");

    let mut buff = [0; 128];
    match TcpListener::bind("127.0.0.1:7834") {
        Ok(listener) => {
            println!("Listening on 127.0.0.1:7834...");
            for mut streamResult in listener.incoming() {
                match &mut streamResult {
                    Ok(stream) => {
                        parse_incoming(stream, &mut buff);
                    }
                    Err(_) => {
                        println!("Error in stream.");
                    }
                }
            }
        }
        Err(_) => {
            println!("Failed to bind");
        }
    }
}