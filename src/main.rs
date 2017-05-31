
extern crate clap;

mod api;
mod reader;

use clap::{Arg, App};

fn main() {
  let matches = App::new("Dummy JSON Server")
                    .version("1.0")
                    .author("sourcepirate")
                    .about("Fake JSON server")
                    .arg(Arg::with_name("jsonfile")
                               .short("f")
                               .long("jsonfile")
                               .value_name("FILE")
                               .help("Sets the json file to serve")
                               .takes_value(true))
                    .arg(Arg::with_name("port")
                              .short("p")
                              .long("port")
                              .value_name("8000")
                              .help("Sets the port to serve")
                              .takes_value(false))
                    .get_matches();
    let port = matches.value_of("port").unwrap_or("8000");
    let file = matches.value_of("jsonfile").unwrap_or(".");
    
    println!("{:?} serving in {:?}", file, port);
}
