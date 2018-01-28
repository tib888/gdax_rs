extern crate gdax_rs;
extern crate tokio_core;

use tokio_core::reactor::Core;

use gdax_rs::RESTClient;
use gdax_rs::products::{GetProductOrderBook, Level};

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut test_client = RESTClient::default(&handle);
    let order_book = core.run(test_client.send_request(&GetProductOrderBook::new(
        String::from("BTC-USD"),
        Level::Top50,
    ))).unwrap();

    println!("{:?}", order_book);
}
