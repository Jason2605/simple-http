pub fn find_content_type(file_extension: &str) -> &str {
    println!("{}", file_extension);
    match file_extension {
        "html" => "Content-Type: text/html;",
        "png" => "Content-Type: image/png",
        "css" => "Content-Type: text/css",
        "js" => "Content-Type: text/javascript",
        "json" => "Content-Type: application/json",
        _ => "Content-Type: text/plain; charset=utf-8",
    }
}
