extern crate gdax_rs;
extern crate tokio_core;

use tokio_core::reactor::Core;

use gdax_rs::RESTClient;
use gdax_rs::products::Get24hrStats;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut test_client = RESTClient::default(&handle);
    let day_stats = core.run(test_client.send_request(&Get24hrStats::new(String::from("BTC-USD"))))
        .unwrap();

    println!("{:?}", day_stats);
}
