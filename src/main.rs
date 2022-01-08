use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
        
    }
}
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("printing the Request {}", String::from_utf8_lossy(&buffer[..]));
    let get= b"GET / HTTP/1.1";

    let mut content = fs::read_to_string("sample.html").unwrap();
    if buffer.starts_with(get){
        let response = format!(
            "HTTP/1.1 202 Accepted\rContent-length {}n\r\n\r\n{}",
            content.len(),
            content
        );
        stream.write(response.as_bytes()).unwrap();
       
    }else{
        content="Not Found".to_string();
        let response = format!(
            "HTTP/1.1 404 Accepted\rContent-length {}n\r\n\r\n{}",
            content.len(),
            content
        );
        stream.write(response.as_bytes()).unwrap();
    }
    stream.flush().unwrap();

    
}
