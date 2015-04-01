use std::collections::HashMap;

use hyper::Url;
use hyper::client::{Client, IntoUrl, Response};
use hyper::header::ContentType;
use hyper::status::StatusCode;
use rustc_serialize::json::Json;
use url::ParseError;

use error::{FleetError, FleetResult};

pub struct API {
    root_url: String,
}

impl API {
    pub fn new(root_url: &'static str) -> Result<API, ParseError> {
        let url = try!(Url::parse(root_url));

        let api = API {
            root_url: format!("{}{}", url.serialize(), "fleet/v1"),
        };

        Ok(api)
    }

    pub fn destroy_unit(&self, name: &str) -> FleetResult<()> {
        let url = &self.url(&format!("/units/{}", name))[..];
        let mut response = try!(self.delete(url));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn get_machines(&self) -> FleetResult<Vec<Json>> {
        let url = &self.url("/machines")[..];
        let mut response = try!(self.get(url));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                Ok(json.find("machines").unwrap().as_array().unwrap().clone())
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn get_unit(&self, name: &str) -> FleetResult<Json> {
        let url = &self.url(&format!("/units/{}", name))[..];
        let mut response = try!(self.get(url));

        match response.status {
            StatusCode::Ok => {
                Ok(Json::from_reader(&mut response).unwrap())
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn get_unit_states(&self, query_pairs: HashMap<&str, &str>) -> FleetResult<Vec<Json>> {
        let base_url = &self.url("/state")[..];
        let mut url = Url::parse(base_url).unwrap();
        url.set_query_from_pairs(query_pairs.iter().map(|(k, v)| (*k, *v)));
        let mut response = try!(self.get(url));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                Ok(json.find("states").unwrap().as_array().unwrap().clone())
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn get_units(&self) -> FleetResult<Vec<Json>> {
        let url = &self.url("/units")[..];
        let mut response = try!(self.get(url));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                Ok(json.find("units").unwrap().as_array().unwrap().clone())
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn put_unit(&self, name: &'static str, body: &str) -> FleetResult<()> {
        let url = &self.url(&format!("/units/{}", name))[..];
        let mut response = try!(self.put(url, body));

        match response.status {
            StatusCode::Created => Ok(()),
            StatusCode::NoContent => Ok(()),
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    fn delete(&self, url: &str) -> FleetResult<Response> {
        let mut client = Client::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        match client.delete(url).header(content_type).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(FleetError::from_hyper_error(&error)),
        }
    }

    fn get<U: IntoUrl>(&self, url: U) -> FleetResult<Response> {
        let mut client = Client::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        match client.get(url).header(content_type).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(FleetError::from_hyper_error(&error)),
        }
    }

    fn put(&self, url: &str, body: &str) -> FleetResult<Response> {
        let mut client = Client::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        match client.put(url).header(content_type).body(body).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(FleetError::from_hyper_error(&error)),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.root_url, path)
    }
}
