extern crate hyper;


use api::API;
use std::sync::{Mutex, Arc};
use self::hyper::server::{Handler, Server, Request, Response};
use self::hyper::method::Method;
use self::hyper::uri::RequestUri;


pub struct DummyServer{
    pub resources: Arc<Mutex<API>>
}

impl Handler for DummyServer {
    fn handle(&self, req: Request, res: Response) {
        let resource = match self.resources.lock(){
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };
        println!("{:?}, {:?}", req.method, req.uri);
        let res_string = resource.get_by_path(req.uri).unwrap();
        res.send(res_string.as_bytes()).unwrap()
    }
}


pub fn get_server(bind: String, content: API){
    let myhandler : DummyServer = DummyServer{
       resources: Arc::new(Mutex::new(content))
    };
    Server::http(&bind[..]).unwrap().handle(myhandler).unwrap();
}
