extern crate gdax_rs;
extern crate tokio_core;

extern crate csv;
extern crate serde;

use std::env;
use tokio_core::reactor::Core;

use gdax_rs::{Cursor, Pagination, RESTClient};
use gdax_rs::products::GetTrades;

fn download_trade_history(
    core: &mut Core,
    client: &mut RESTClient,
    product: &str,
    path: &str,
    startid: Option<usize>,
) {
    println!(
        "Downloading {} starting at {:?} and writing {} please be patient...",
        &product, startid, &path
    );
    let mut wrt = csv::Writer::from_path(path).unwrap();
    let mut page = if let Some(lastid) = startid {
        Some(Pagination {
            page: Cursor::After(lastid),
            limit: None,
        })
    } else {
        None
    };

    loop {
        if let Ok(trades) =
            core.run(client.send_request(&GetTrades::new(String::from(product), page)))
        {
            if let Some(last) = trades.last() {
                page = Some(Pagination {
                    page: Cursor::After(last.trade_id),
                    limit: None,
                });
            } else {
                break;
            };

            for trade in trades {
                wrt.serialize(trade).unwrap();
            }
        }
    }

    wrt.flush().unwrap();
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut test_client = RESTClient::default(&handle);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let (startid, postfix) = if args.len() > 2 {
            (Some(args[2].parse().unwrap()), args[2].to_owned())
        } else {
            (None, "".to_owned())
        };

        let path = if args.len() > 3 {
            args[3].to_owned()
        } else {
            format!("{}-{}.csv", args[1], postfix).to_owned()
        };

        download_trade_history(&mut core, &mut test_client, &args[1], &path, startid);
    } else {
        println!("Example usage: download_trade_history.exe BTC-USD 1000 BTC-USD.csv");
    }
}
