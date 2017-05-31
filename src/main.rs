
extern crate clap;

mod api;
mod reader;

use clap::{Arg, App, ArgMatches};
use clap::AppSettings;

fn main() {
  let app = App::new("Dummy JSON Server")
                    .version("1.0")
                    .author("sourcepirate")
                    .about("Fake JSON server")
                    .setting(AppSettings::ArgRequiredElseHelp)
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
                              .takes_value(true));

    let matches: ArgMatches = app.get_matches();
    let port = matches.value_of("port").unwrap_or("8000");

    if let Some(name) = matches.value_of("jsonfile") {
      println!("{:?} serving in {:?}", name, port);
    }
}
