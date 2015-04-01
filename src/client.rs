use std::collections::{BTreeMap, HashMap};

use hyper::Client as HyperClient;
use hyper::client::{IntoUrl, Response};
use hyper::header::ContentType;
use hyper::status::StatusCode;
use rustc_serialize::json::{self, Json, ToJson};
use url::{ParseError, Url};

use error::{FleetError, FleetResult};
use schema::{Machine, Unit, UnitOption, UnitState, UnitStates};
use serialize::{CreateUnit, ModifyUnit};

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
            StatusCode::Ok => Ok(()),
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn get_unit(&self, name: &str) -> Result<Unit, FleetError> {
        let url = self.build_url(&format!("/units/{}", name));
        let mut response = try!(self.get(&url[..]));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();

                Ok(self.unit_from_json(&json))
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

                Ok(units_json.iter().map(|json| self.machine_from_json(json)).collect())
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

                Ok(unit_states_json.iter().map(|json| self.unit_state_from_json(json)).collect())
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

                Ok(units_json.iter().map(|json| self.unit_from_json(json)).collect())
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

    fn get_metadata_hashmap(&self, json_obj: &BTreeMap<String, Json>) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        match json_obj.get("metadata") {
            Some(metadata_json) => {
                let metadata_json_obj = metadata_json.as_object().unwrap();

                for (key, value) in metadata_json_obj.iter() {
                    metadata.insert(key.clone(), value.as_string().unwrap().to_string());
                }

                metadata
            },
            None => metadata,
        }
    }

    fn get_string_value<'a>(&'a self, json_obj: &'a BTreeMap<String, Json>, key: &str) -> &str {
        json_obj.get(key).unwrap().as_string().unwrap()
    }

    fn machine_from_json(&self, json: &Json) -> Machine {
        let machine_obj = json.as_object().unwrap();

        Machine {
            id: self.get_string_value(machine_obj, "id").to_string(),
            metadata: self.get_metadata_hashmap(machine_obj),
            primary_ip: self.get_string_value(machine_obj, "primaryIP").to_string(),
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

    fn unit_from_json(&self, json: &Json) -> Unit {
        let unit_obj = json.as_object().unwrap();

        Unit {
            current_state: UnitStates::from_str(self.get_string_value(unit_obj, "currentState")),
            desired_state: UnitStates::from_str(self.get_string_value(unit_obj, "desiredState")),
            machine_id: self.get_string_value(unit_obj, "machineID").to_string(),
            name: self.get_string_value(unit_obj, "name").to_string(),
            options: unit_obj.get("options").unwrap().as_array().unwrap().iter().map(|opt_json| {
                self.unit_option_from_json(opt_json)
            }).collect(),
        }
    }

    fn unit_option_from_json(&self, json: &Json) -> UnitOption {
        let unit_obj = json.as_object().unwrap();

        UnitOption {
            name: self.get_string_value(unit_obj, "name").to_string(),
            section: self.get_string_value(unit_obj, "section").to_string(),
            value: self.get_string_value(unit_obj, "value").to_string(),
        }
    }

    fn unit_state_from_json(&self, json: &Json) -> UnitState {
        let unit_obj = json.as_object().unwrap();

        UnitState {
            name: self.get_string_value(unit_obj, "name").to_string(),
            hash: self.get_string_value(unit_obj, "hash").to_string(),
            machine_id: self.get_string_value(unit_obj, "machineID").to_string(),
            systemd_load_state: self.get_string_value(unit_obj, "systemdLoadState").to_string(),
            systemd_active_state: self.get_string_value(unit_obj, "systemdActiveState").to_string(),
            systemd_sub_state: self.get_string_value(unit_obj, "systemdSubState").to_string(),
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
