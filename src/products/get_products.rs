use hyper::Method;

use serde_util::deserialize_from_str;
use rest_client::{EndPointRequest, RestRequest};
use url::Route;

/// This struct is the request handler
#[derive(Default)]
pub struct GetProducts;

impl GetProducts {
    pub fn new() -> GetProducts {
        GetProducts::default()
    }
}

// TODO: use builder pattern instead of pub field?
/// This struct represent the response of GDAX API, each field of the json are parsed.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Product {
    pub id: String,
    pub base_currency: String,
    pub quote_currency: String,
    #[serde(deserialize_with = "deserialize_from_str")] pub base_min_size: f64,
    #[serde(deserialize_with = "deserialize_from_str")] pub base_max_size: f64,
    #[serde(deserialize_with = "deserialize_from_str")] pub quote_increment: f64,
}

impl EndPointRequest<Vec<Product>> for GetProducts {
    fn create_request(&self) -> RestRequest {
        RestRequest {
            http_method: Method::Get,
            route: Route::new().add_segment(&"/products"),
            body: String::new(),
            pagination: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use hyper::Method;
    use serde_json;

    use super::{EndPointRequest, GetProducts, Product, RestRequest, Route};

    #[test]
    fn test_create_request() {
        let handler = GetProducts::new();

        let expected = RestRequest {
            http_method: Method::Get,
            route: Route::new().add_segment(&"/products"),
            body: String::new(),
            pagination: None,
        };

        assert_eq!(handler.create_request(), expected);
    }

    #[test]
    fn test_deserialize() {
        let result: Vec<Product> = serde_json::from_str(
            r#"[
                {
                    "id": "BTC-USD",
                    "base_currency": "BTC",
                    "quote_currency": "USD",
                    "base_min_size": "0.01",
                    "base_max_size": "10000.00",
                    "quote_increment": "0.01"
                }
            ]"#,
        ).unwrap();
        let expected: Vec<Product> = vec![
            Product {
                id: String::from("BTC-USD"),
                base_currency: String::from("BTC"),
                quote_currency: String::from("USD"),
                base_min_size: 0.01,
                base_max_size: 10000.0,
                quote_increment: 0.01,
            },
        ];
        assert_eq!(result, expected);
    }
}
