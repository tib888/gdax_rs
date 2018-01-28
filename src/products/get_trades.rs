use chrono::{DateTime, Utc};
use hyper::Method;

use serde_util::deserialize_from_str;
use rest_client::{EndPointRequest, Pagination, RestRequest};
use url::Route;

pub struct GetTrades {
    product_id: String,
    pagination: Option<Pagination>,
}

impl GetTrades {
    pub fn new(product_id: String, pagination: Option<Pagination>) -> GetTrades {
        GetTrades {
            product_id: product_id,
            pagination: pagination,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Side {
    #[serde(rename = "sell")] Sell,
    #[serde(rename = "buy")] Buy,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Trade {
    pub time: DateTime<Utc>,
    pub trade_id: usize,
    #[serde(deserialize_with = "deserialize_from_str")] pub price: f64,
    #[serde(deserialize_with = "deserialize_from_str")] pub size: f64,
    pub side: Side,
}

impl EndPointRequest<Vec<Trade>> for GetTrades {
    fn create_request(&self) -> RestRequest {
        RestRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&self.product_id)
                .add_segment(&"trades"),
            body: String::new(),
            pagination: self.pagination.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use serde_json;

    use super::{EndPointRequest, GetTrades, Method, RestRequest, Route, Side, Trade};

    #[test]
    fn test_create_request() {
        let result = GetTrades::new(String::from("BTC-USD"), None).create_request();
        let expected = RestRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&"BTC-USD")
                .add_segment(&"trades"),
            body: String::new(),
            pagination: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result: Vec<Trade> = serde_json::from_str(
            "\
[{
    \"time\": \"2014-11-07T22:19:28.578544Z\",
    \"trade_id\": 74,
    \"price\": \"10.00000000\",
    \"size\": \"0.01000000\",
    \"side\": \"buy\"
}, {
    \"time\": \"2014-11-07T01:08:43.642366Z\",
    \"trade_id\": 73,
    \"price\": \"100.00000000\",
    \"size\": \"0.01000000\",
    \"side\": \"sell\"
}]",
        ).unwrap();
        let expected = vec![
            Trade {
                time: Utc.ymd(2014, 11, 07).and_hms_micro(22, 19, 28, 578_544),
                trade_id: 74,
                price: 10.0,
                size: 0.01,
                side: Side::Buy,
            },
            Trade {
                time: Utc.ymd(2014, 11, 07).and_hms_micro(1, 8, 43, 642_366),
                trade_id: 73,
                price: 100.0,
                size: 0.01,
                side: Side::Sell,
            },
        ];

        assert_eq!(result, expected);
    }
}
