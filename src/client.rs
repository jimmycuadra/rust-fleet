use std::collections::{BTreeMap, HashMap};

use rustc_serialize::json::{self, Json};

use schema::{Machine, Unit, UnitOption, UnitState, UnitStates};
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

    pub fn destroy_unit(&self, name: &str) -> Result<(), String> {
        self.fleet.destroy_unit(name)
    }

    pub fn get_unit(&self, name: &str) -> Result<Unit, String> {
        match self.fleet.get_unit(name) {
            Ok(json) => Ok(self.unit_from_json(&json)),
            Err(error) => Err(error),
        }
    }

    pub fn list_machines(&self) -> Result<Vec<Machine>, String> {
        match self.fleet.get_machines() {
            Ok(units_json) => {
                Ok(units_json.iter().map(|json| self.machine_from_json(json)).collect())
            },
            Err(error) => Err(error),
        }
    }

    pub fn list_unit_states(
        &self,
        machine_id: Option<&str>,
        unit_name: Option<&str>
    ) -> Result<Vec<UnitState>, String> {
        let mut query_pairs = HashMap::new();

        if machine_id.is_some() {
            query_pairs.insert("machineID", machine_id.unwrap());
        }

        if unit_name.is_some() {
            query_pairs.insert("unitName", unit_name.unwrap());
        }

        match self.fleet.get_unit_states(query_pairs) {
            Ok(unit_states_json) => {
                Ok(unit_states_json.iter().map(|json| self.unit_state_from_json(json)).collect())
            },
            Err(error) => Err(error),
        }
    }

    pub fn list_units(&self) -> Result<Vec<Unit>, String> {
        match self.fleet.get_units() {
            Ok(units_json) => Ok(units_json.iter().map(|json| self.unit_from_json(json)).collect()),
            Err(error) => Err(error),
        }
    }

    pub fn modify_unit(&self, name: &'static str, desired_state: UnitStates) -> Result<(), String> {
        let mut body = HashMap::new();

        body.insert("desiredState", desired_state.to_json());

        self.fleet.put_unit(name, &json::encode(&body).unwrap())
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
