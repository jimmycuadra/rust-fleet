extern crate fleet;

use fleet::{Client, UnitOption, UnitStates};

#[test]
fn create_unit() {
    let client = Client::new("http://localhost:2999").unwrap();
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

#[test]
fn create_invalid_unit_missing_name() {
    let client = Client::new("http://localhost:2999").unwrap();
    let options = vec![];

    let result = client.create_unit("", UnitStates::Launched, options);
    let error = result.err().unwrap();

    assert_eq!(format!("{}", error), "404: Error in JSON response from Fleet was empty");
}

#[test]
fn create_invalid_unit_missing_options() {
    let client = Client::new("http://localhost:2999").unwrap();
    let options = vec![];

    let result = client.create_unit("optionless.service", UnitStates::Launched, options);
    let error = result.err().unwrap();

    assert_eq!(format!("{}", error), "409: unit does not exist and options field empty");
}
