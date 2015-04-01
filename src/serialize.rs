use std::collections::{BTreeMap, HashMap};

use rustc_serialize::json::Json;

use schema::{Machine, Unit, UnitOption, UnitState, UnitStates};

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
pub struct CreateUnit {
    pub desiredState: Json,
    pub options: Vec<UnitOption>,
}

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
pub struct ModifyUnit {
    pub desiredState: Json,
}

pub fn get_metadata_hashmap(json_obj: &BTreeMap<String, Json>) -> HashMap<String, String> {
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

pub fn get_string_value<'a>(json_obj: &'a BTreeMap<String, Json>, key: &str) -> &'a str {
    json_obj.get(key).unwrap().as_string().unwrap()
}

pub fn machine_from_json(json: &Json) -> Machine {
    let machine_obj = json.as_object().unwrap();

    Machine {
        id: get_string_value(machine_obj, "id").to_string(),
        metadata: get_metadata_hashmap(machine_obj),
        primary_ip: get_string_value(machine_obj, "primaryIP").to_string(),
    }
}

pub fn unit_from_json(json: &Json) -> Unit {
    let unit_obj = json.as_object().unwrap();

    Unit {
        current_state: UnitStates::from_str(get_string_value(unit_obj, "currentState")),
        desired_state: UnitStates::from_str(get_string_value(unit_obj, "desiredState")),
        machine_id: get_string_value(unit_obj, "machineID").to_string(),
        name: get_string_value(unit_obj, "name").to_string(),
        options: unit_obj.get("options").unwrap().as_array().unwrap().iter().map(|opt_json| {
            unit_option_from_json(opt_json)
        }).collect(),
    }
}

pub fn unit_option_from_json(json: &Json) -> UnitOption {
    let unit_obj = json.as_object().unwrap();

    UnitOption {
        name: get_string_value(unit_obj, "name").to_string(),
        section: get_string_value(unit_obj, "section").to_string(),
        value: get_string_value(unit_obj, "value").to_string(),
    }
}

pub fn unit_state_from_json(json: &Json) -> UnitState {
    let unit_obj = json.as_object().unwrap();

    UnitState {
        name: get_string_value(unit_obj, "name").to_string(),
        hash: get_string_value(unit_obj, "hash").to_string(),
        machine_id: get_string_value(unit_obj, "machineID").to_string(),
        systemd_load_state: get_string_value(unit_obj, "systemdLoadState").to_string(),
        systemd_active_state: get_string_value(unit_obj, "systemdActiveState").to_string(),
        systemd_sub_state: get_string_value(unit_obj, "systemdSubState").to_string(),
    }
}
