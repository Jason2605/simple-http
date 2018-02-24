# simple-http
A very simple threaded web server implemented in the rust programming language.

### Installing and running simple-http

```
$ git clone https://github.com/Jason2605/simple-http.git
$ cd simple-http
$ cargo run
```

simple-http has 3 optional arguments:
- -p Set the port to bind too (default 8080)
- -tp The amount of threads to spawn into the thread pool (default 4)
- -wd The working directory to read the web files from (default html)

simple-http will allow for a file to be directly found, by this i mean http://127.0.0.1:8080/index.html will work, however you can define routes [here](https://github.com/Jason2605/simple-http/blob/master/src/handle_request/routes/mod.rs) and allow the extension to be dropped.
