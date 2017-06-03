## Dummy

An Experimental cli JSON mock server entirely dedicated for me to learn rust. Serves any json through the traversal path.

## Installing

Make sure cargo is installed on your machine.

```
cargo build

```

Once the binary is generated.

```
cd target/debug

./dummy -f json_file_to_server.json -p 8080

```

Serves the json file on port 8080. You can check using curl

```

curl -v http://localhost:8080

```

## Resources

* [hyper](https://hyper.rs/hyper/v0.10.11/hyper/index.html)
* [serde](https://docs.serde.rs/serde_json/)
* [clap-rs](https://docs.rs/clap/2.24.2/clap/)


## Contribution

* You are welcome to contribute. Give me a PR at any time.


## License
MIT


