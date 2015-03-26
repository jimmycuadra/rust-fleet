use hyper::client::{Client, Response};
use hyper::header::ContentType;
use hyper::status::StatusCode;
use rustc_serialize::json::Json;

pub struct FleetAPI {
    root_url: &'static str,
}

impl FleetAPI {
    pub fn new(root_url: &'static str) -> FleetAPI {
        FleetAPI {
            root_url: root_url
        }
    }

    pub fn get_unit(&self, name: &str) -> Result<Json, String> {
        let url = &self.url(&format!("/units/{}", name))[..];
        let mut response = self.get(url);

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                Ok(json.find("unit").unwrap().clone())
            },
            StatusCode::NotFound => Err("Unit not found".to_string()),
            status_code => Err(format!("Unexpected response: {}", status_code)),
        }
    }

    pub fn get_units(&self) -> Result<Vec<Json>, String> {
        let url = &self.url("/units")[..];
        let mut response = self.get(url);

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                Ok(json.find("units").unwrap().as_array().unwrap().clone())
            },
            status_code => Err(format!("Unexpected response: {}", status_code)),
        }
    }

    pub fn put_unit(&self, name: &'static str, body: &str) -> Result<(), String> {
        let url = &self.url(&format!("/units/{}", name))[..];
        let response = self.put(url, body);

        match response.status {
            StatusCode::Created => Ok(()),
            StatusCode::NoContent => Ok(()),
            StatusCode::Conflict => Err("UnitOptions are required".to_string()),
            StatusCode::BadRequest => Err("Invalid unit".to_string()),
            status_code => Err(format!("Unexpected response: {}", status_code))
        }
    }

    fn get(&self, url: &str) -> Response {
        let mut client = Client::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        client.get(url).header(content_type).send().unwrap()
    }

    fn put(&self, url: &str, body: &str) -> Response {
        let mut client = Client::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        client.put(url).header(content_type).body(body).send().unwrap()
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.root_url, path)
    }
}
