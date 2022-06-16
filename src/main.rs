use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener:TcpListener = TcpListener::bind("127.0.0.1:7879").unwrap();
    
    for stream in listener.incoming(){
        let stream:TcpStream = stream.unwrap();
        
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer: [u8;1024] = [0;1024];

    stream.read(&mut buffer).unwrap();
    /* header
    println!(
        "Request : {}",
        String::from_utf8_lossy(&buffer[..])); */
    let response : &str = "HTTP/1.1 200 OK/r/n/r/n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
