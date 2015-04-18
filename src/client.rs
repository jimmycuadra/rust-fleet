use std::collections::HashMap;

use hyper::Client as HyperClient;
use hyper::client::{IntoUrl, Response};
use hyper::header::ContentType;
use hyper::status::StatusCode;
use rustc_serialize::json::{self, Json, ToJson};
use url::{ParseError, Url};

use error::FleetError;
use schema::{MachinePage, Unit, UnitOption, UnitPage, UnitStatePage, UnitStates};
use serialize::{self, CreateUnit, ModifyUnit};

/// An API client for fleet.
///
/// # Examples
///
/// ```
/// extern crate fleet;
///
/// use fleet::Client;
///
/// let client = Client::new("http://localhost:2999");
///
/// match client.list_units() {
///     Ok(unit_page) => {
///         for unit in unit_page.units {
///             println!("{}", unit.name);
///         }
///     },
///     None => println!("No units in fleet!"),
/// };
pub struct Client {
    root_url: String,
}

impl Client {
    /// Constructs a new `Client`.
    ///
    /// `root_url` is a network scheme, hostname or IP address, and optional port where fleetd is
    /// running. This value should not include a path.
    ///
    /// Due to limitations in underlying libraries, Unix domain sockets are not yet supported. On
    /// CoreOS, fleet runs only on a Unix domain socket by default. It can be exposed on a TCP port
    /// by including a systemd drop-in for the `fleet.socket` unit. This can be achieved via
    /// cloud-config by overriding the default `fleet.socket` unit.
    ///
    /// ```yaml
    /// #cloud-config
    ///
    /// ---
    /// coreos:
    ///   units:
    ///     -  name: fleet.socket
    ///        command: start
    ///        drop_ins:
    ///        -  name: 30-ListenStream.conf
    ///           content: |
    ///             [Socket]
    ///             ListenStream=2999
    /// ```
    ///
    /// # Failures
    ///
    /// If the value provided for `root_url` cannot be parsed, a `url::ParseError` will be
    /// returned.
    pub fn new(root_url: &str) -> Result<Client, ParseError> {
        let url = try!(Url::parse(root_url));
        let client = Client {
            root_url: format!("{}{}", url.serialize(), "fleet/v1"),
        };

        Ok(client)
    }

    /// Creates a fleet unit.
    ///
    /// A unit consists of a name, the desired runtime state, and a set of unit options which
    /// comprise the unit's unit file.
    ///
    /// # Failures
    ///
    /// Returns a `FleetError` if there is a problem with the HTTP request to the API or the API
    /// itself returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern create fleet;
    /// # use fleet::{Client, UnitOption};
    /// # let client = Client::new("http://localhost:2999").unwrap();
    /// let options = vec![
    ///     UnitOption {
    ///         name: "ExecStart".to_string(),
    ///         section: "Service".to_string(),
    ///         value: "/usr/bin/sleep 3000".to_string(),
    ///     },
    /// ];
    ///
    /// client.create_unit("test.service", UnitStates::Launched, options).unwrap();
    /// ```
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

    /// Destroys the unit with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fleet;
    /// # use fleet::Client;
    /// # let client = Client::new("http://localhost:2999");
    /// client.destroy_unit("test.service").ok().unwrap();
    pub fn destroy_unit(&self, name: &str) -> Result<(), FleetError> {
        let url = self.build_url(&format!("/units/{}", name));
        let mut response = try!(self.delete(&url[..]));

        match response.status {
            StatusCode::NoContent => Ok(()),
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    /// Gets a single unit by name.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fleet;
    /// # use fleet::Client;
    /// # let client = Client::new("http://localhost:2999");
    /// client.get_unit("test.service").ok().unwrap();
    /// ```
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

    pub fn list_machines(&self) -> Result<MachinePage, FleetError> {
        let url = self.build_url(&format!("/machines"));
        let mut response = try!(self.get(&url[..]));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();

                let machines = match json.find("machines") {
                    Some(machines_json) => {
                        let machines = machines_json.as_array().unwrap();

                        machines.iter().map(|json| {
                            serialize::machine_from_json(json)
                        }).collect()
                    },
                    None => vec![],
                };

                Ok(MachinePage {
                    machines: machines,
                    next_page_token: self.get_next_page_token(&json),
                })
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn list_unit_states(
        &self,
        machine_id: Option<&str>,
        unit_name: Option<&str>
    ) -> Result<UnitStatePage, FleetError> {
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

                let unit_states = match json.find("states") {
                    Some(unit_states_json) => {
                        let unit_states = unit_states_json.as_array().unwrap();

                        unit_states.iter().map(|json| {
                            serialize::unit_state_from_json(json)
                        }).collect()
                    },
                    None => vec![],
                };

                Ok(UnitStatePage {
                    states: unit_states,
                    next_page_token: self.get_next_page_token(&json),
                })
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    pub fn list_units(&self) -> Result<UnitPage, FleetError> {
        let url = self.build_url("/units");
        let mut response = try!(self.get(&url[..]));

        match response.status {
            StatusCode::Ok => {
                let json = Json::from_reader(&mut response).unwrap();
                let units_json = json.find("units").unwrap().as_array().unwrap();
                let units = units_json.iter().map(|json| serialize::unit_from_json(json)).collect();

                Ok(UnitPage {
                    units: units,
                    next_page_token: self.get_next_page_token(&json),
                })
            },
            _ => Err(FleetError::from_hyper_response(&mut response)),
        }
    }

    /// Modifies a unit, instructing fleetd to move the unit to a new state.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate fleet;
    /// # use fleet::{Client, UnitStates};
    /// # let client = Client::new("http://localhost:2999");
    /// client.modify_unit("test.service", UnitStates::Loaded).ok().unwrap();
    /// ```
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

    fn delete(&self, url: &str) -> Result<Response, FleetError> {
        let mut client = HyperClient::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        match client.delete(url).header(content_type).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(FleetError::from_hyper_error(&error)),
        }
    }

    fn get<U: IntoUrl>(&self, url: U) -> Result<Response, FleetError> {
        let mut client = HyperClient::new();
        let content_type: ContentType = ContentType("application/json".parse().unwrap());

        match client.get(url).header(content_type).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(FleetError::from_hyper_error(&error)),
        }
    }

    fn get_next_page_token(&self, json: &Json) -> Option<String> {
        match json.find("nextPageToken") {
            Some(next_page_token_json) => match next_page_token_json.as_string() {
                Some(next_page_token) => Some(next_page_token.to_string()),
                None => None,
            },
            None => None,
        }
    }

    fn put(&self, url: String, body: String) -> Result<Response, FleetError> {
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
