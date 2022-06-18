use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::fs;
fn main() {
    let listener:TcpListener = TcpListener::bind("127.0.0.1:7879").unwrap();
    
    for stream in listener.incoming(){
        let stream:TcpStream = stream.unwrap();
        thread::sleep(Duration::from_secs(5));
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
    let sleep : &[u8;21] = b"GET /sleep HTTP/1.1\r\n";


    let (status_line , filename) = 
            if buffer.starts_with(get) {
                ("HTTP/1.1 200 OK","index.html")
            }else if buffer.starts_with(sleep){
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK","index.html")
            }else{
                ("HTTP/1.1 404 NOT FOUND","Error.html")
            };
    let contents: String = fs::read_to_string(format!("template/{}",filename)).unwrap();
    
    let response : String = format!("{}\r\nContent-lentgh : {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    /* let responseView:String =  format!("Response en bytes : {}",response);
    println!("{:?}",responseView.as_bytes()); */
    
}
