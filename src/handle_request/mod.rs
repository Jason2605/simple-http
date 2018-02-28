extern crate chrono;

use std::net::TcpStream;
use std::io::prelude::*;
use std::str;
use self::chrono::prelude::*;

pub mod status;
pub mod compress;
pub mod routes;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let http_stream = str::from_utf8(&buffer).unwrap();

    if http_stream != &"\u{0}".repeat(512) {
        let (route, http_request_type, compress) = parse_http(http_stream);
        if http_request_type.to_str() != "UNKNOWN" {
            println!("[{}] {:?} {} {}",
                Utc::now().format("%d/%B/%Y %H:%M:%S"),
                stream.peer_addr().unwrap().ip(),
                &http_request_type.to_str(),
                &route,
            );

            let mut response = routes::find_route(route, http_request_type);
            if compress {
                compress::compress(&mut response);
            }
            routes::respond::create_headers(&mut response, compress);

            stream.write(&response.contents).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn parse_http<'a>(http_stream: &'a str) -> (&'a str, status::RequestType, bool) {

    let http_verb = http_stream.split_whitespace().next().unwrap();
    let http_request_type = status::RequestType::from_str(http_verb);
    let http_request: Vec<&str> = http_stream.split_whitespace().collect();
    let compress = http_stream.to_lowercase().contains("accept-encoding: gzip");
    (http_request[1], http_request_type, compress)
}


#[cfg(test)]
mod tests {
    use super::parse_http;
    use super::status::RequestType;

    #[test]
    fn parse_http_head_no_gzip() {
        let (route, request_type, compress) = parse_http("HEAD /");

        assert_eq!(route, "/");
        assert_eq!(request_type, RequestType::HEAD);
        assert_eq!(compress, false);
    }

    #[test]
    fn parse_http_post_no_gzip() {
        let (route, request_type, compress) = parse_http("POST /api/user/create");

        assert_eq!(route, "/api/user/create");
        assert_eq!(request_type, RequestType::POST);
        assert_eq!(compress, false);
    }

    #[test]
    fn parse_http_put_with_gzip() {
        let (route, request_type, compress) = parse_http("PUT /api/user/edit \nAccept-Encoding: gzip");

        assert_eq!(route, "/api/user/edit");
        assert_eq!(request_type, RequestType::PUT);
        assert_eq!(compress, true);
    }

    #[test]
    fn parse_http_get_with_gzip() {
        let (route, request_type, compress) = parse_http("GET / \nAccept-Encoding: gzip");

        assert_eq!(route, "/");
        assert_eq!(request_type, RequestType::GET);
        assert_eq!(compress, true);
    }

    #[test]
    fn parse_http_unknown_no_gzip() {
        let (route, request_type, compress) = parse_http("TEAPOT /im/a/teapot");

        assert_eq!(route, "/im/a/teapot");
        assert_eq!(request_type, RequestType::UNKNOWN);
        assert_eq!(compress, false)
    }
}
