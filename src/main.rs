use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

struct Response {
    method: Method,
    status: u16,
    version: String,
    // data: Vec<u8>
    headers: Vec<String>,
}

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0; 500];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response =
    b"HTTP/1.1 200 OK\r\nContent-Type: text/json; charset=ISO-8859-1\r\n\r\n[{\"data\": {\"0\": 1}}]";
    if let Err(e) = stream.write_all(response) {
        println!("Failed sending response: {}", e)
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming().filter_map(Result::ok) {
        spawn(|| handle_client(stream));
    }
    Ok(())
}
