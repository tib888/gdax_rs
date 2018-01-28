extern crate gdax_rs;
extern crate tokio_core;

use tokio_core::reactor::Core;

use gdax_rs::RESTClient;
use gdax_rs::currencies::GetCurrencies;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut test_client = RESTClient::default(&handle);
    let currencies = core.run(test_client.send_request(&GetCurrencies::new()))
        .unwrap();

    println!("{:?}", currencies);
}
