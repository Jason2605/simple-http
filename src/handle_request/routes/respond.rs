use std::fs::File;
use std::io::prelude::*;
use std::str;

use handle_request::status::StatusCodes;
use handle_request::routes::content_types;

pub fn read_file(filename: &str) -> Vec<u8> {
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
    contents = create_headers(status_code, contents, content_type);
    contents
}

fn create_headers(status_code: StatusCodes, contents: Vec<u8>, content_type: &str) -> Vec<u8> {
    let status_line = format!("HTTP/1.1 {} {} \n{}\r\n\r\n",
        status_code.to_u16(),
        status_code.to_str(),
        content_type
    );
    let status_line = status_line.as_bytes();

    let mut return_contents = status_line.to_vec();
    return_contents.extend(contents);
    return_contents
}

pub fn json_string(contents: &str) -> Vec<u8> {
    let json = format!("HTTP/1.1 200 OK \nContent-Type: application/json;\r\n\r\n{}", contents);
    json.into_bytes()
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
