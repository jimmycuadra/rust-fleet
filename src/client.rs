use std::collections::HashMap;

use hyper::Client as HyperClient;
use hyper::client::{IntoUrl, Response};
use hyper::header::ContentType;
use hyper::status::StatusCode;
use rustc_serialize::json::{self, Json, ToJson};
use url::{ParseError, Url};

use error::{FleetError, FleetResult};
use schema::{Machine, Unit, UnitOption, UnitState, UnitStates};
use serialize::{self, CreateUnit, ModifyUnit};

pub struct Client {
    root_url: String,
}

impl Client {
    pub fn new(root_url: &str) -> Result<Client, ParseError> {
        let url = try!(Url::parse(root_url));
        let client = Client {
            root_url: format!("{}{}", url.serialize(), "fleet/v1"),
        };

        Ok(client)
    }

    pub fn create_unit(
        &self,
        name: &str,
        desired_state: UnitStates,
        options: Vec<UnitOption>
    ) -> Result<(), FleetError> {
        let serializer = CreateUnit {
            desiredState: desired_state.to_json(),
            options: options,
        };

        let url = self.build_url(&format!("/units/{}", name));
        let body = json::encode(&serializer).unwrap();
        let mut response = try!(self.put(url, body));

        match response.status {
            StatusCode::Created | StatusCode::NoContent => Ok(()),
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn destroy_unit(&self, name: &str) -> Result<(), FleetError> {
        let url = self.build_url(&format!("/units/{}", name));
        let mut response = try!(self.delete(&url[..]));

        match response.status {
            StatusCode::NoContent => Ok(()),
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn get_unit(&self, name: &str) -> Result<Unit, FleetError> {
        let url = self.build_url(&format!("/units/{}", name));
        let mut response = try!(self.get(&url[..]));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();

                Ok(serialize::unit_from_json(&json))
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn list_machines(&self) -> Result<Vec<Machine>, FleetError> {
        let url = self.build_url(&format!("/machines"));
        let mut response = try!(self.get(&url[..]));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                let units_json = json.find("machines").unwrap().as_array().unwrap();

                Ok(units_json.iter().map(|json| serialize::machine_from_json(json)).collect())
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn list_unit_states(
        &self,
        machine_id: Option<&str>,
        unit_name: Option<&str>
    ) -> Result<Vec<UnitState>, FleetError> {
        let mut query_pairs = HashMap::new();

        if machine_id.is_some() {
            query_pairs.insert("machineID", machine_id.unwrap());
        }

        if unit_name.is_some() {
            query_pairs.insert("unitName", unit_name.unwrap());
        }

        let base_url = self.build_url("/state");
        let mut url = Url::parse(&base_url[..]).unwrap();
        url.set_query_from_pairs(query_pairs.iter().map(|(k, v)| (*k, *v)));
        let mut response = try!(self.get(url));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                let unit_states_json = json.find("states").unwrap().as_array().unwrap();

                Ok(unit_states_json.iter().map(|json| {
                    serialize::unit_state_from_json(json)
                }).collect())
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn list_units(&self) -> Result<Vec<Unit>, FleetError> {
        let url = self.build_url("/units");
        let mut response = try!(self.get(&url[..]));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                let units_json = json.find("units").unwrap().as_array().unwrap();

                Ok(units_json.iter().map(|json| serialize::unit_from_json(json)).collect())
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn modify_unit(
        &self,
        name: &str,
        desired_state: UnitStates
    ) -> Result<(), FleetError> {
        let serializer = ModifyUnit {
            desiredState: desired_state.to_json(),
        };

        let url = self.build_url(&format!("/units/{}", name));
        let body = json::encode(&serializer).unwrap();
        let mut response = try!(self.put(url, body));

        match response.status {
            StatusCode::Created | StatusCode::NoContent => Ok(()),
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    // Private

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.root_url, path)
    }

    fn delete(&self, url: &str) -> FleetResult<Response> {
        let mut client = HyperClient::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        match client.delete(url).header(content_type).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(FleetError::from_hyper_error(&error)),
        }
    }

    fn get<U: IntoUrl>(&self, url: U) -> FleetResult<Response> {
        let mut client = HyperClient::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        match client.get(url).header(content_type).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(FleetError::from_hyper_error(&error)),
        }
    }

    fn put(&self, url: String, body: String) -> FleetResult<Response> {
        let mut client = HyperClient::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        match client.put(&url[..]).header(content_type).body(&body[..]).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(FleetError::from_hyper_error(&error)),
        }
    }
}

#[cfg(test)]
mod client_tests {
    use super::Client;

    #[test]
    fn it_can_be_constructed() {
       Client::new("http://localhost").unwrap();
    }

    #[test]
    fn it_returns_an_error_for_invalid_root_urls() {
        assert!(Client::new("asdf").is_err());
    }
}
