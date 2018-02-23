use std::fs::File;
use std::io::prelude::*;
use std::str;

use handle_request::status::StatusCodes;
use handle_request::routes::content_types;

pub struct Response {
    pub status_code: StatusCodes,
    pub content_type: String,
    pub contents: Vec<u8>,
}

pub fn read_file(filename: &str) -> Response {

    let mut contents = Vec::new();
    let (mut file, status_code) = match File::open(format!("html/{}", filename)) {
        Ok(f) => (f, StatusCodes::Ok),
        Err(_) => (File::open("html/404.html").unwrap(), StatusCodes::NotFound)
    };
    file.read_to_end(&mut contents).unwrap();
    let content_type = if status_code.is_404() {
        "Content-Type: text/html;"
    } else {
        content_types::find_content_type(filename.split(".").last().unwrap())
    };

    let response = Response {
        status_code,
        content_type: String::from(content_type),
        contents,
    };

    response
}

pub fn create_headers(response: &mut Response, compress: bool) {

    let compress_header = if compress {
        "\nContent-Encoding: gzip"
    } else {
        ""
    };

    let status_line = format!("HTTP/1.1 {} {} {}\n{}\r\n\r\n",
        response.status_code.to_u16(),
        response.status_code.to_str(),
        compress_header,
        response.content_type
    );
    let status_line = status_line.as_bytes();

    let mut return_contents = status_line.to_vec();
    return_contents.extend(&response.contents);
    response.contents = return_contents;
}

pub fn json_string(contents: &str) -> Response {
    Response {
        status_code: StatusCodes::Ok,
        content_type: String::from("Content-Type: application/json;"),
        contents: format!("HTTP/1.1 200 OK \nContent-Type: application/json;\r\n\r\n{}", contents).into_bytes(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_a_file() {
        let contents = "HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    <title>Test!</title>\n  </head>\n  <body>\n    <h1>Test!</h1>\n    <p>Hi from Rust!!!!</p>\n  </body>\n</html>\n";
        assert_eq!(read_file("test.html"), contents);
    }

    #[test]
    fn test_create_headers_200() {
        let contents = String::from("test");
        assert_eq!(create_headers(false, contents), "HTTP/1.1 200 OK\r\n\r\ntest");
    }

    #[test]
    fn test_create_headers_404() {
        let contents = String::from("test");
        assert_eq!(create_headers(true, contents), "HTTP/1.1 404 NOT FOUND\r\n\r\ntest");
    }

    #[test]
    fn test_create_json_response() {
        let contents = String::from("{\"test\": 10}");
        assert_eq!(json_string(contents), "HTTP/1.1 200 OK \nContent-Type: application/json;\r\n\r\n{\"test\": 10}");
    }
}
