use hyper::Method;
use serde_json;

use ::{EndPointRequestHandler, EndPointRequest};
use ::deserialize_from_str;

pub struct GetProducts;

impl GetProducts {
    pub fn new() -> GetProducts {
        GetProducts {}
    }
}

// TODO: use builder pattern instead of pub field?
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Product {
    pub id: String,
    pub base_currency: String,
    pub quote_currency: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base_min_size: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base_max_size: f64,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub quote_increment: f64
}

impl EndPointRequestHandler<Vec<Product>> for GetProducts {
    fn create_request(&self) -> EndPointRequest {
        EndPointRequest {
            http_method: Method::Get,
            route: String::from("/products"),
            body: String::new(),
        }
    }

    fn deserialize(&self, http_body: String) -> Vec<Product> {
        println!("{}", &http_body);
        serde_json::from_str(&http_body).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use hyper::Method;

    use super::{GetProducts, Product};
    use EndPointRequestHandler;
    use EndPointRequest;

    #[test]
    fn test_request() {
        let handler = GetProducts::new();

        let expected = EndPointRequest {
            http_method: Method::Get,
            route: String::from("/products"),
            body: String::new(),
        };

        assert_eq!(handler.create_request(), expected);
    }

    #[test]
    fn test_deserialize() {
        let result = GetProducts.deserialize(String::from("
[
    {
        \"id\": \"BTC-USD\",
        \"base_currency\": \"BTC\",
        \"quote_currency\": \"USD\",
        \"base_min_size\": \"0.01\",
        \"base_max_size\": \"10000.00\",
        \"quote_increment\": \"0.01\"
    }
]
        "));
        let expected: Vec<Product> = vec![
            Product {
                id: String::from("BTC-USD"),
                base_currency: String::from("BTC"),
                quote_currency: String::from("USD"),
                base_min_size: 0.01,
                base_max_size: 10000.0,
                quote_increment: 0.01,
            }
        ];
        assert_eq!(result, expected);
    }
}
