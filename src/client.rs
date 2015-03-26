use std::collections::HashMap;

use rustc_serialize::json;

use schema::{UnitOption, UnitStates};
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
    ) -> Result<(), &str> {
        let options_json = json::encode(&options).unwrap();
        let mut body = HashMap::new();

        body.insert("desiredState", desired_state.to_json());
        body.insert("options", &options_json);

        self.fleet.put_units(name, &json::encode(&body).unwrap())
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
