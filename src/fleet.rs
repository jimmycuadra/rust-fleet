use hyper::client::{Client, Response};
use hyper::header::ContentType;
use hyper::status::StatusCode;

pub struct FleetAPI {
    root_url: &'static str,
}

impl FleetAPI {
    pub fn new(root_url: &'static str) -> FleetAPI {
        FleetAPI {
            root_url: root_url
        }
    }

    pub fn put_unit(&self, name: &'static str, body: &str) -> Result<(), &'static str> {
        let url = &self.url(format!("/units/{}", name))[..];
        let response = self.put(url, body);

        match response.status {
            StatusCode::Created => Ok(()),
            StatusCode::Conflict => Err("UnitOptions are required"),
            StatusCode::BadRequest => Err("Invalid unit"),
            status_code => panic!("Unexpected response: {}", status_code)
        }
    }

    fn put(&self, url: &str, body: &str) -> Response {
        let mut client = Client::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        client.put(url).header(content_type).body(body).send().unwrap()
    }

    fn url(&self, path: String) -> String {
        format!("{}{}", self.root_url, path)
    }
}
