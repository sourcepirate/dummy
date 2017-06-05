## Dummy

An Experimental CLI JSON mock server entirely dedicated for me to learn rust. Serves any json through the traversal path.

## Installing

Make sure cargo is installed on your machine. 
To install cargo, you may follow: https://www.rust-lang.org/en-US/other-installers.html

```
cargo build

```

Once the binary is generated.

```
cd target/debug

./dummy  -f ../../sample/json_file_to_serve.json -p 8080

```

Serves the json file on port 8080. You can check using curl

```

curl -v http://localhost:8080

```

## Resources

* [hyper](https://hyper.rs/hyper/v0.10.11/hyper/index.html)
* [serde](https://docs.serde.rs/serde_json/)
* [clap-rs](https://docs.rs/clap/2.24.2/clap/)

## Benchmarking information

I tried using loadtest npm to benchmark the json server.

```
$  loadtest -c 10 --rps 200 http://localhost:8080/posts

[Sat Jun 03 2017 22:39:45 GMT+0530 (IST)] INFO Requests: 0, requests per second: 0, mean latency: 0 ms
[Sat Jun 03 2017 22:39:50 GMT+0530 (IST)] INFO Requests: 909, requests per second: 182, mean latency: 2.7 ms
[Sat Jun 03 2017 22:39:55 GMT+0530 (IST)] INFO Requests: 1907, requests per second: 200, mean latency: 2.5 ms
[Sat Jun 03 2017 22:40:00 GMT+0530 (IST)] INFO Requests: 2907, requests per second: 200, mean latency: 2.3 ms
[Sat Jun 03 2017 22:40:05 GMT+0530 (IST)] INFO Requests: 3907, requests per second: 200, mean latency: 2.1 ms
[Sat Jun 03 2017 22:40:10 GMT+0530 (IST)] INFO Requests: 4907, requests per second: 200, mean latency: 2.4 ms
[Sat Jun 03 2017 22:40:15 GMT+0530 (IST)] INFO Requests: 5907, requests per second: 200, mean latency: 2.2 ms
[Sat Jun 03 2017 22:40:20 GMT+0530 (IST)] INFO Requests: 6907, requests per second: 200, mean latency: 2.3 ms
[Sat Jun 03 2017 22:40:25 GMT+0530 (IST)] INFO Requests: 7907, requests per second: 200, mean latency: 2.9 ms
[Sat Jun 03 2017 22:40:30 GMT+0530 (IST)] INFO Requests: 8907, requests per second: 200, mean latency: 2.4 ms
[Sat Jun 03 2017 22:40:35 GMT+0530 (IST)] INFO Requests: 9907, requests per second: 200, mean latency: 2.5 ms
[Sat Jun 03 2017 22:40:40 GMT+0530 (IST)] INFO Requests: 10907, requests per second: 200, mean latency: 2.4 ms
[Sat Jun 03 2017 22:40:45 GMT+0530 (IST)] INFO Requests: 11907, requests per second: 200, mean latency: 2.3 ms
[Sat Jun 03 2017 22:40:50 GMT+0530 (IST)] INFO Requests: 12907, requests per second: 200, mean latency: 2.2 ms
[Sat Jun 03 2017 22:40:55 GMT+0530 (IST)] INFO Requests: 13907, requests per second: 200, mean latency: 2.2 ms
[Sat Jun 03 2017 22:41:00 GMT+0530 (IST)] INFO Requests: 14907, requests per second: 200, mean latency: 2.3 ms
[Sat Jun 03 2017 22:41:05 GMT+0530 (IST)] INFO Requests: 15907, requests per second: 200, mean latency: 2.5 ms
[Sat Jun 03 2017 22:41:10 GMT+0530 (IST)] INFO Requests: 16907, requests per second: 200, mean latency: 2.6 ms
[Sat Jun 03 2017 22:41:15 GMT+0530 (IST)] INFO Requests: 17907, requests per second: 200, mean latency: 3 ms
[Sat Jun 03 2017 22:41:20 GMT+0530 (IST)] INFO Requests: 18907, requests per second: 200, mean latency: 2.4 ms

```


## Contribution

* You are welcome to contribute. Give me a PR at any time.


## License
MIT


