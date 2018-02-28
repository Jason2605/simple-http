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
    use super::content_types::find_content_type;
    use super::create_headers;
    use super::Response;
    use super::StatusCodes;

    #[test]
    fn create_headers_status_ok_html_no_gzip() {
        let content_type = find_content_type("html");
        let mut response = Response {
            status_code: StatusCodes::Ok,
            content_type: String::from(content_type),
            contents: "hello, world".as_bytes().to_vec()
        };
        create_headers(&mut response, false);

        assert_eq!(response.status_code, StatusCodes::Ok);
        assert_eq!(response.content_type, content_type);
        assert_eq!(response.contents, "HTTP/1.1 200 OK \nContent-Type: text/html\r\n\r\nhello, world".as_bytes().to_vec());
    }

    #[test]
    fn create_headers_status_not_found_js_no_gzip() {
        let content_type = find_content_type("js");
        let mut response = Response {
            status_code: StatusCodes::NotFound,
            content_type: String::from(content_type),
            contents: "console.log('hello');".as_bytes().to_vec()
        };
        create_headers(&mut response, false);

        assert_eq!(response.status_code, StatusCodes::NotFound);
        assert_eq!(response.content_type, content_type);
        assert_eq!(response.contents, "HTTP/1.1 404 Not Found \nContent-Type: text/javascript\r\n\r\nconsole.log('hello');".as_bytes().to_vec())
    }

    #[test]
    fn create_headers_status_unknown_css_no_gzip() {
        let content_type = find_content_type("css");
        let mut response = Response {
            status_code: StatusCodes::Unknown,
            content_type: String::from(content_type),
            contents: ".test {}".as_bytes().to_vec()
        };
        create_headers(&mut response, false);

        assert_eq!(response.status_code, StatusCodes::Unknown);
        assert_eq!(response.content_type, content_type);
        assert_eq!(response.contents, "HTTP/1.1 0 Unknown \nContent-Type: text/css\r\n\r\n.test {}".as_bytes().to_vec());
    }

    #[test]
    fn create_headers_status_ok_png_with_gzip() {
        let content_type = find_content_type("png");
        let mut response = Response {
            status_code: StatusCodes::Ok,
            content_type: String::from(content_type),
            contents: "beep".as_bytes().to_vec()
        };
        create_headers(&mut response, true);

        assert_eq!(response.status_code, StatusCodes::Ok);
        assert_eq!(response.content_type, content_type);
        assert_eq!(response.contents, "HTTP/1.1 200 OK \nContent-Encoding: gzip\nContent-Type: image/png\r\n\r\nbeep".as_bytes().to_vec());
    }

    #[test]
    fn create_headers_status_not_found_json_with_gzip() {
        let content_type = find_content_type("json");
        let mut response = Response {
            status_code: StatusCodes::NotFound,
            content_type: String::from(content_type),
            contents: "[]".as_bytes().to_vec()
        };
        create_headers(&mut response, true);

        assert_eq!(response.status_code, StatusCodes::NotFound);
        assert_eq!(response.content_type, content_type);
        assert_eq!(response.contents, "HTTP/1.1 404 Not Found \nContent-Encoding: gzip\nContent-Type: application/json\r\n\r\n[]".as_bytes().to_vec());
    }

    #[test]
    fn create_headers_status_unknown_plain_with_gzip() {
        let content_type = find_content_type("whatever");
        let mut response = Response {
            status_code: StatusCodes::Unknown,
            content_type: String::from(content_type),
            contents: "??".as_bytes().to_vec()
        };
        create_headers(&mut response, true);

        assert_eq!(response.status_code, StatusCodes::Unknown);
        assert_eq!(response.content_type, content_type);
        assert_eq!(response.contents, "HTTP/1.1 0 Unknown \nContent-Encoding: gzip\nContent-Type: text/plain; charset=utf-8\r\n\r\n??".as_bytes().to_vec());
    }
}
