use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

use rustc_serialize::json::{self, Json};

use schema::{Unit, UnitOption, UnitStates};
use fleet::FleetAPI;

pub struct Client {
    fleet: FleetAPI,
}

impl Client {
    pub fn new(root_url: &'static str) -> Client {
        Client {
            fleet: FleetAPI::new(root_url)
        }
    }

    pub fn create_unit(
        &self,
        name: &'static str,
        desired_state: UnitStates,
        options: Vec<UnitOption>
    ) -> Result<(), String> {
        let options_json = json::encode(&options).unwrap();
        let mut body = HashMap::new();

        body.insert("desiredState", desired_state.to_json());
        body.insert("options", &options_json);

        self.fleet.put_unit(name, &json::encode(&body).unwrap())
    }

    pub fn list_units(&self) -> Result<Vec<Unit>, String> {
        match self.fleet.get_units() {
            Ok(units_json) => {
                Ok(units_json.iter().map(|json| self.unit_from_json(json)).collect())
            },
            Err(error) => Err(error),
        }
    }

    pub fn modify_unit(&self, name: &'static str, desired_state: UnitStates) -> Result<(), String> {
        let mut body = HashMap::new();

        body.insert("desiredState", desired_state.to_json());

        self.fleet.put_unit(name, &json::encode(&body).unwrap())
    }

    fn get_string_value<'a>(&'a self, json_obj: &'a BTreeMap<String, Json>, key: &str) -> &str {
        json_obj.get(key).unwrap().as_string().unwrap()
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
}

#[cfg(test)]
mod client_tests {
    use super::Client;
    use schema::UnitStates;

    #[test]
    fn it_can_be_constructed() {
       Client::new("http://localhost");
    }

    #[test]
    #[should_panic]
    fn it_can_create_units() {
        let client = Client::new("http://it_can_create_units.example.com");

        let result = client.create_unit("example.service", UnitStates::Launched, vec![]);

        assert!(result.is_ok())
    }
}
