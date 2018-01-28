//! This module contains all `EndPointRequest` and there response type of GDAX API doc under
//! "Market Data/Currencies" section (<https://docs.gdax.com/#currencies>)

use hyper::Method;

use serde_util::deserialize_from_str;
use rest_client::{EndPointRequest, RestRequest};
use url::Route;

#[derive(Default)]
pub struct GetCurrencies;

impl GetCurrencies {
    pub fn new() -> GetCurrencies {
        GetCurrencies::default()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Currency {
    id: String,
    name: String,
    #[serde(deserialize_with = "deserialize_from_str")] min_size: f64,
}

impl EndPointRequest<Vec<Currency>> for GetCurrencies {
    fn create_request(&self) -> RestRequest {
        RestRequest {
            http_method: Method::Get,
            route: Route::new().add_segment(&"currencies"),
            body: String::new(),
            pagination: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use hyper::Method;
    use serde_json;

    use super::{Currency, EndPointRequest, GetCurrencies, RestRequest, Route};

    #[test]
    fn test_create_request() {
        let result = GetCurrencies::new().create_request();

        let expected = RestRequest {
            http_method: Method::Get,
            route: Route::new().add_segment(&"currencies"),
            body: String::new(),
            pagination: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result: Vec<Currency> = serde_json::from_str(
            "[{
    \"id\": \"BTC\",
    \"name\": \"Bitcoin\",
    \"min_size\": \"0.00000001\"
}, {
    \"id\": \"USD\",
    \"name\": \"United States Dollar\",
    \"min_size\": \"0.01000000\"
}]",
        ).unwrap();
        let expected = vec![
            Currency {
                id: String::from("BTC"),
                name: String::from("Bitcoin"),
                min_size: 0.00000001,
            },
            Currency {
                id: String::from("USD"),
                name: String::from("United States Dollar"),
                min_size: 0.01,
            },
        ];

        assert_eq!(result, expected);
    }
}
