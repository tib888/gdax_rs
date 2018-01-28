extern crate gdax_rs;
extern crate tokio_core;

extern crate csv;
extern crate serde;
// extern crate serde_derive;
// use std::error::Error;
// use std::fs::File;
// use csv::Writer;

use std::time::{SystemTime, UNIX_EPOCH};

use tokio_core::reactor::Core;

use gdax_rs::{Cursor, Pagination, RESTClient};
use gdax_rs::time::GetTime;
// use gdax_rs::currencies::GetCurrencies;
use gdax_rs::products::{Get24hrStats, GetProductOrderBook, GetProductTicker, GetProducts,
                        GetTrades, Level};

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut test_client = RESTClient::default(&handle);

    let time = core.run(test_client.send_request(&GetTime::new())).unwrap();
    let local_epoch_diff = time.epoch
        - SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64;
    println!(
        "GDAX Time: {} Local time is late with: {}",
        time.iso, local_epoch_diff
    );

    // let _currencies = core.run(test_client.send_request(&GetCurrencies::new()))
    //     .unwrap();

    let products = core.run(test_client.send_request(&GetProducts::new()))
        .unwrap();

    for product in products {
        // let order_book = core.run(
        //     test_client.send_request(&GetProductOrderBook::new(product.id.clone(), Level::Best)),
        // ).unwrap();

        let last_ticker = core.run(
            test_client.send_request(&GetProductTicker::new(product.id.clone())),
        ).unwrap();

        // let product_trades = core.run(
        //     test_client.send_request(&GetTrades::new(product.id.clone())),
        // ).unwrap();

        // let day_stats = core.run(
        //     test_client.send_request(&Get24hrStats::new(product.id.clone())),
        // ).unwrap();

        println!(
            "{}\tprice: {}\tvolume: {}\ttime: {}",
            product.id, last_ticker.price, last_ticker.volume, last_ticker.time
        );
    }
}
