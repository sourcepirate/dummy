extern crate hyper;

use self::hyper::server::{Server, Request, Response};
use self::hyper::method::Method;
use self::hyper::uri::RequestUri;

pub fn handler(req: Request, res: Response) {
    if let RequestUri::AbsolutePath(x) = req.uri {
        println!("{:?}", x);
    } else {
        println!("{:?}", req.uri);
    }
}


pub fn get_server(){
    Server::http("0.0.0.0:8080").unwrap().handle(handler).unwrap();
}
