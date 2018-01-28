use hyper::Method;

use serde_util::deserialize_from_str;
use rest_client::{EndPointRequest, RestRequest};
use url::Route;
use uuid::Uuid;
use serde;
use serde_json;

/// This struct represent the endpoint `Get Product Order Book` <https://docs.gdax.com/#get-product-order-book>
/// (The level 3 requests are not implemented)
pub struct GetProductOrderBook {
    pub product_id: String,
    pub level: Level,
}

/// This enum represents the order book possible levels to request.
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum Level {
    /// Only the best bid and ask (aggregated)
    /// The size field is the sum of the size of the orders at that price, and num-orders is the count of orders at that price; size should not be multiplied by num-orders.
    Best = 1,

    /// Top 50 bids and asks (aggregated)
    /// The size field is the sum of the size of the orders at that price, and num-orders is the count of orders at that price; size should not be multiplied by num-orders.
    Top50 = 2,

    /// Full order book (non aggregated)
    /// Level 3 is only recommended for users wishing to maintain a full real-time order book using the websocket stream. Abuse of Level 3 via polling will cause your access to be limited or blocked.
    Full = 3, // TODO: Handle level 3 (with enum)
}

impl GetProductOrderBook {
    pub fn new(product_id: String, level: Level) -> GetProductOrderBook {
        GetProductOrderBook { product_id, level }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderBook<T> {
    pub sequence: usize,
    pub bids: Vec<T>,
    pub asks: Vec<T>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum OrderInfo {
    #[serde(rename = "num_orders")] Count(i64),
    #[serde(rename = "order_id")] Id(Uuid),
}

fn deserialize_orderinfo<'de, D>(deserializer: D) -> Result<OrderInfo, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deser_result: serde_json::Value = try!(serde::Deserialize::deserialize(deserializer));

    if let Some(count) = deser_result.as_i64() {
        return Ok(OrderInfo::Count(count));
    }

    if let Some(s) = deser_result.as_str() {
        if let Ok(id) = Uuid::parse_str(s) {
            return Ok(OrderInfo::Id(id));
        }
    }

    Err(serde::de::Error::custom("Unexpected value"))
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PriceLevel {
    #[serde(deserialize_with = "deserialize_from_str")] pub price: f64,
    #[serde(deserialize_with = "deserialize_from_str")] pub size: f64,
    #[serde(deserialize_with = "deserialize_orderinfo")] pub orderinfo: OrderInfo,
}

impl EndPointRequest<OrderBook<PriceLevel>> for GetProductOrderBook {
    fn create_request(&self) -> RestRequest {
        RestRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&self.product_id)
                .add_segment(&"book")
                .add_attribute_value(&"level", &(self.level as i32)),
            body: String::new(),
            pagination: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use hyper::Method;
    use serde_json;
    use uuid::Uuid;

    use super::{EndPointRequest, GetProductOrderBook, Level, OrderBook, OrderInfo, PriceLevel,
                RestRequest, Route};

    #[test]
    fn test_create_request() {
        let request_handler = GetProductOrderBook::new(String::from("BTC-USD"), Level::Top50);
        let result = request_handler.create_request();
        let expected = RestRequest {
            http_method: Method::Get,
            route: Route::new()
                .add_segment(&"products")
                .add_segment(&"BTC-USD")
                .add_segment(&"book")
                .add_attribute_value(&"level", &2),
            body: String::new(),
            pagination: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result: OrderBook<PriceLevel> = serde_json::from_str(
            "
{
    \"sequence\": 3,
    \"bids\": [
        [\"16839.45\",\"0.47037038\",2],
        [\"16835.39\",\"0.00075522\",2]
    ],
    \"asks\": [
        [\"16913.21\",\"4.85\",1],
        [\"16918.01\",\"0.70301839\",11],
        [\"16918.02\",\"9.88197274\",24]
    ]
}
        ",
        ).unwrap();
        let expected = OrderBook {
            sequence: 3,
            bids: vec![
                PriceLevel {
                    price: 16839.45,
                    size: 0.47037038,
                    orderinfo: OrderInfo::Count(2),
                },
                PriceLevel {
                    price: 16835.39,
                    size: 0.00075522,
                    orderinfo: OrderInfo::Count(2),
                },
            ],
            asks: vec![
                PriceLevel {
                    price: 16913.21,
                    size: 4.85,
                    orderinfo: OrderInfo::Count(1),
                },
                PriceLevel {
                    price: 16918.01,
                    size: 0.70301839,
                    orderinfo: OrderInfo::Count(11),
                },
                PriceLevel {
                    price: 16918.02,
                    size: 9.88197274,
                    orderinfo: OrderInfo::Count(24),
                },
            ],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn test_deserialize_level3() {
        let result: OrderBook<PriceLevel> = serde_json::from_str(
            "
{
    \"sequence\": 3,
    \"bids\": [
        [\"16839.45\",\"0.47037038\",\"3b0f1225-7f84-490b-a29f-0faef9de823a\"],
        [\"16835.39\",\"0.00075522\",\"3b0f1225-7f84-490b-a29f-1faef9de823a\"]
    ],
    \"asks\": [
        [\"16913.21\",\"4.85\",\"da863862-25f4-4868-ac41-005d11ab0a5f\"],
        [\"16918.01\",\"0.70301839\",\"da863862-25f4-4868-ac41-005d11ab1a5f\"],
        [\"16918.02\",\"9.88197274\",\"da863862-25f4-4868-ac41-005d11ab2a5f\"]
    ]
}
        ",
        ).unwrap();
        let expected = OrderBook {
            sequence: 3,
            bids: vec![
                PriceLevel {
                    price: 16839.45,
                    size: 0.47037038,
                    orderinfo: OrderInfo::Id(
                        Uuid::parse_str("3b0f1225-7f84-490b-a29f-0faef9de823a").unwrap(),
                    ),
                },
                PriceLevel {
                    price: 16835.39,
                    size: 0.00075522,
                    orderinfo: OrderInfo::Id(
                        Uuid::parse_str("3b0f1225-7f84-490b-a29f-1faef9de823a").unwrap(),
                    ),
                },
            ],
            asks: vec![
                PriceLevel {
                    price: 16913.21,
                    size: 4.85,
                    orderinfo: OrderInfo::Id(
                        Uuid::parse_str("da863862-25f4-4868-ac41-005d11ab0a5f").unwrap(),
                    ),
                },
                PriceLevel {
                    price: 16918.01,
                    size: 0.70301839,
                    orderinfo: OrderInfo::Id(
                        Uuid::parse_str("da863862-25f4-4868-ac41-005d11ab1a5f").unwrap(),
                    ),
                },
                PriceLevel {
                    price: 16918.02,
                    size: 9.88197274,
                    orderinfo: OrderInfo::Id(
                        Uuid::parse_str("da863862-25f4-4868-ac41-005d11ab2a5f").unwrap(),
                    ),
                },
            ],
        };
        assert_eq!(result, expected)
    }
}
