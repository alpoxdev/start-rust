// extern crate communicator;
extern crate project8;

pub mod a {
	pub mod series {
			pub mod of {
					pub fn nested_modules(x: String) {
						println!("{}", x);
					}
			}
	}
}

use a::series::of::nested_modules;
use project8::*;

fn main(){
	client::connect();
	// communicator::network::connect();
	// communicator::network::server::connect();

	a::series::of::nested_modules(String::from("Hello, World!"));
	nested_modules(String::from("Use Use!"));
}
