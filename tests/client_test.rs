extern crate fleet;

use fleet::{Client, UnitOption, UnitStates};

#[test]
fn unit_lifecycle() {
    let client = Client::new("http://localhost:2999").unwrap();

    // Create unit

    let options = vec![
        UnitOption {
            name: "ExecStart".to_string(),
            section: "Service".to_string(),
            value: "/usr/bin/sleep 3000".to_string(),
        },
    ];

    let create_result = client.create_unit("test.service", UnitStates::Launched, options);

    assert!(create_result.is_ok(), "{}", create_result.err().unwrap());

    // Get unit

    let unit = client.get_unit("test.service").ok().unwrap();

    assert_eq!(&unit.name[..], "test.service");

    // Modify unit's desired state

    let modify_result = client.modify_unit("test.service", UnitStates::Loaded);

    assert!(modify_result.is_ok(), "{}", modify_result.err().unwrap());

    // Destroy unit

    let destroy_result = client.destroy_unit("test.service");

    assert!(destroy_result.is_ok(), "{}", destroy_result.err().unwrap());
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
