#[derive(Debug)]
pub enum StatusCodes {
    Ok,
    NotFound,
    Unknown,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RequestType {
    GET,
    POST,
    PUT,
    HEAD,
    //PRI,
    UNKNOWN,
}

impl StatusCodes {
    pub fn from_u16(value: u16) -> StatusCodes {
        match value {
            200 => StatusCodes::Ok,
            404 => StatusCodes::NotFound,
            _ => StatusCodes::Unknown,
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            StatusCodes::Ok => 200,
            StatusCodes::NotFound => 404,
            StatusCodes::Unknown => 0,
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match *self {
            StatusCodes::Ok => "OK",
            StatusCodes::NotFound => "Not Found",
            StatusCodes::Unknown => "Unkown",
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
    pub fn from_str(value: &str) -> RequestType {
        match value {
            "GET" => RequestType::GET,
            "POST" => RequestType::POST,
            "PUT" => RequestType::PUT,
            "HEAD" => RequestType::HEAD,

            //"PRI" => RequestType::PRI,
            _ => RequestType::UNKNOWN,
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match *self {
            RequestType::GET => "GET",
            RequestType::POST => "POST",
            RequestType::PUT => "PUT",
            RequestType::HEAD => "HEAD",

            //RequestType::PRI => "PRI",
            RequestType::UNKNOWN => "UNKNOWN",
        }
    }
}
