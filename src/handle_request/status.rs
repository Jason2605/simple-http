#[derive(Debug)]
pub enum StatusCodes {
    Ok,
    NotFound,
}

#[derive(Debug)]
pub enum RequestType {
    GET,
    POST,
    PUT,
    UNKNOWN,
}

impl StatusCodes {
    pub fn from_u16(value: u16) -> StatusCodes {
        match value {
            200 => StatusCodes::Ok,
            404 => StatusCodes::NotFound,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            StatusCodes::Ok => 200,
            StatusCodes::NotFound => 404,
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match *self {
            StatusCodes::Ok => "OK",
            StatusCodes::NotFound => "Not Found",
        }
    }

    pub fn is_404(&self) -> bool {
        match *self {
            StatusCodes::NotFound => true,
            _ => false,
        }
    }
}

impl RequestType {
    pub fn from_str(value: &'static str) -> RequestType {
        match value {
            "GET" => RequestType::GET,
            "POST" => RequestType::POST,
            "PUT" => RequestType::PUT,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match *self {
            RequestType::GET => "GET",
            RequestType::POST => "POST",
            RequestType::PUT => "PUT",
            RequestType::UNKNOWN => panic!("Ok"),
        }
    }
}
