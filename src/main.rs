use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
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
    let get : &[u8;16] = b"GET / HTTP/1.1\r\n"; 
    
    if buffer.starts_with(get){
        let contents : String = fs::read_to_string("template/index.html").unwrap();
        let response : String = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(),
                        contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else{
        let status_line:&str = "HTTP/1.1 404 NOT FOUND";

        let contents: String = fs::read_to_string("template/Error.html").unwrap();
        let response : String = format!("{}\r\nContent-lentgh : {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        let responseView:String =  format!("Response en bytes : {}",response);
        println!("{:?}",responseView.as_bytes());

    }
    
    
}
