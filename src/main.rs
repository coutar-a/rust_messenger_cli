extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Client, Uri};
use hyper::header::{Connection, Headers, UserAgent};
use tokio_core::reactor::Core;


//Hyper wrapper, meant to simplify everything
struct Httprequest {

	core: tokio_core::reactor::Core,
	client: hyper::Client<hyper::client::HttpConnector>,
	get_fn: fn(&mut Httprequest, &str)
}


//get method
fn test_get(obj : &mut Httprequest, target : &str) {

	let url : Uri = target.parse().unwrap();
	let mut test = hyper::Request::new(hyper::Method::Get, url);
	
	//gotta find a way to pass headers
	test.headers_mut().set(UserAgent::new("curl/7.55.1"));

 let request = obj.client.request(test).and_then(|res| {
        print!("Response: ");
        res.body().concat2().and_then(move |body| {
     	io::stdout().write_all(&body).ok();
     	Ok(())
     })
    });

 obj.core.run(request).unwrap();
}

//implementation
impl Httprequest {
	fn new(_core : tokio_core::reactor::Core, _client : hyper::Client<hyper::client::HttpConnector>) -> Httprequest {
		Httprequest {
			core : _core,
			client : _client,
			get_fn: test_get
		}
	}

	//helper function for get_fn
	fn get(&mut self, target : &str) {
		(self.get_fn)(self, target);
	}
}

//Can't reference core in new for client::new(), so I need an helper function to instanciate the damn thing
fn init_request() -> Httprequest {

	let core = Core::new().unwrap();
	let client = Client::new(&core.handle());
	return Httprequest::new(core, client);

}

fn main() {

	let mut client = init_request();
	client.get("http://www.wttr.in/Paris");
}
