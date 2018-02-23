extern crate deflate;

use handle_request::routes::respond;
use self::deflate::deflate_bytes_gzip;

pub fn compress(response: &mut respond::Response) {
    response.contents = deflate_bytes_gzip(&response.contents[..]);
}
