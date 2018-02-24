use handle_request::status::RequestType;

pub mod respond;

pub mod content_types;

pub fn find_route<'a>(route: &'a str, http_request_type: RequestType) -> respond::Response {

    //Ensure they arent trying a directory traversal
    if route.contains("./") || route.contains("../") {
        return respond::read_file("404.html")
    }

    match (route, http_request_type) {
        ("/", RequestType::GET) => respond::read_file("index.html"),
        ("/home", RequestType::GET) => respond::read_file("home.html"),
        ("/test", RequestType::POST) => respond::json_string("{\"test\": 10}"),
        _ => respond::read_file(route),
    }
}
