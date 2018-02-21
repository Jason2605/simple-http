extern crate simple_http;
use simple_http::thread::ThreadPool;
use simple_http::handle_request;

use std::net::TcpListener;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let (port, thread_count, directory) = arg_parse(&args);

    let address = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&address).unwrap();

    println!("Server being served at {}...", &address);
    println!("Server running with {} threads in the ThreadPool...", &thread_count);
    println!("Server serving HTML files from {}...", &directory);

    let pool = ThreadPool::new(thread_count).unwrap_or_else(|err| {
        eprintln!("Problem creating ThreadPool: {:?}", err.error);
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_request::handle_connection(stream);
        });
    };
}

fn arg_parse(args: &Vec<String>) -> (&str, usize, &str) {

    let mut port = "8080";
    let mut thread_count: usize = 4;
    let mut directory = "html";

    let mut i = 0;


    while i < args.len() {
        match args[i].as_ref() {
            "-p" => {i += 1; port = args[i].as_ref();},
            "-tp" => {i += 1; thread_count = args[i].parse().unwrap();},
            "-wd" => {i += 1; directory = args[i].as_ref();},
            _ => println!("Unknown argument"),
        };
        i += 1
    }

    (port, thread_count, directory)

}
