//import tcp listener 
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
fn main() {
    let listener = TcpListener::bind( "127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming(){
        let _stream = stream.unwrap();
        
        println!("Connection established!");
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream : TcpStream){
    let mut buffer =[0; 1024];
    
    stream.read(&mut buffer).unwrap();
    
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
