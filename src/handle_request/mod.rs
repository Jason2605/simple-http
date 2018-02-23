use std::net::TcpStream;
use std::io::prelude::*;
use std::str;

pub mod status;
pub mod compress;
pub mod routes;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let http_stream = str::from_utf8(&buffer[..]).unwrap();
    let (route, http_request_type, compress) = parse_http(http_stream);
    let mut response = routes::find_route(route, http_request_type);

    if compress {
        compress::compress(&mut response);
    }
    routes::respond::create_headers(&mut response, compress);
    stream.write(&response.contents).unwrap();
    stream.flush().unwrap();
}

fn parse_http<'a>(http_stream: &'a str) -> (&'a str, status::RequestType, bool) {

    let http_verb = http_stream.split_whitespace().next().unwrap();
    let http_request_type = status::RequestType::from_str(http_verb);
    let http_request: Vec<&str> = http_stream.split_whitespace().collect();
    let compress = http_stream.to_lowercase().contains("accept-encoding: gzip");
    (http_request[1], http_request_type, compress)
}
