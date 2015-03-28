extern crate fleet_client;

use fleet_client::{Client, UnitOption, UnitStates};

#[test]
fn create_unit() {
    let client = Client::new("http://localhost:2999/fleet/v1");
    let options = vec![
        UnitOption {
            name: "ExecStart".to_string(),
            section: "Service".to_string(),
            value: "/usr/bin/sleep 3000".to_string(),
        },
    ];

    let result = client.create_unit("test.service", UnitStates::Launched, options);

    assert!(result.is_ok(), "{}", result.err().unwrap());
}
