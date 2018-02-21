use std::net::TcpStream;
use std::io::prelude::*;
use std::str;

use std::fs::File;

pub mod status;

pub mod routes;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let http_stream = str::from_utf8(&buffer[..]).unwrap();
    let (route, http_request_type) = parse_http(http_stream);
    let contents = routes::find_route(route, http_request_type);

    println!("{:?}", String::from_utf8_lossy(&buffer));

    stream.write(&contents).unwrap();
    stream.flush().unwrap();
}


pub fn read_file_test() -> Vec<u8> {

    let contents_str = "HTTP/1.1 200 OK\r\n\r\n";
    let contents_byte = contents_str.as_bytes();

    let mut contents = Vec::new();
    contents.extend_from_slice(&contents_byte);

    let mut file = match File::open("html/test.html") {
        Ok(f) => f,
        Err(_) => File::open("html/404.html").unwrap()
    };
    file.read_to_end(&mut contents).unwrap();
    contents
}

fn parse_http<'a>(http_stream: &'a str) -> (&'a str, status::RequestType) {

    let http_verb = http_stream.split_whitespace().next();

    let http_request_type = match http_verb.unwrap() {
        "GET" => status::RequestType::GET,
        "POST" => status::RequestType::POST,
        _ => status::RequestType::UNKNOWN,
    };

    let http_request: Vec<&str> = http_stream.split_whitespace().collect();
    (http_request[1], http_request_type)
}
