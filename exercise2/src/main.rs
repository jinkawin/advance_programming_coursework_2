extern crate dns_lookup;

use std::env;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::from_utf8;
use std::sync::mpsc;
use std::thread;

use dns_lookup::lookup_host;

const BUFFER_SIZE: usize = 1024;

fn main(){
    let args: Vec<String> = env::args().collect();
    let hostname = args[1].clone();

    let (tx, rx) = mpsc::channel();
    let mut children = vec![];

    // Get all IP Addresses
    let ips: Vec<std::net::IpAddr> = dnsLookUp(&hostname);

    for ip in ips.clone(){
        let _tx = tx.clone();

        // Send "Sender" and ip to the function
        children.push(thread::spawn(move || {
            tcpConnect(_tx, ip);
        }));
    }

    getRequest(&rx.recv().unwrap(), &hostname);
}

// Find IP addresses by hostname
fn dnsLookUp(hostname: &str) -> Vec<std::net::IpAddr>{
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    ips
}

// Initial the connection
fn tcpConnect(sender: mpsc::Sender<std::net::TcpStream>, ip: std::net::IpAddr){
    let socket = SocketAddr::new(ip, 80); // Create a ner socket for the connection with port 80
    let stream = TcpStream::connect(socket); // Make a connection by using socket

    match stream{
        Ok(s) => {
            println!("Sent: {}", ip);
            sender.send(s); // If it successfully connect, send the connection to thread.
        }
        Err(e) => println!("Error: {}", e)
    }
}

fn getRequest(mut stream: &TcpStream, hostname: &str){

    let msg = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", hostname); // Prepare for the data
    stream.write(msg.as_bytes()); // Write the data to stream

    let mut buf = [0 as u8; BUFFER_SIZE]; // Initial buffer for reading the response

    let result = stream.read(&mut buf); // Read a response
    match result {
        Ok(n) => {
            let text = from_utf8(&buf); // Convert from utf8 to readable text
            match text {
                Err(e) => { println!("Failed to decode: {}", e); }
                Ok(s) => { println!("{}", s); }
            }
        }
        Err(e) => { println!("End: {}", e); }
        _ => {},
    }
}