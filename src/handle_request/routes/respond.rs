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
