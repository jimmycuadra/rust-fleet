extern crate fleet;
extern crate retry;

use retry::retry;

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

    // fleet takes a second to create the unit, so give it a few tries.
    let unit = retry(5, 500, || {
        client.get_unit("test.service").ok().unwrap()
    }, |unit| {
        unit.machine_id.is_some()
    }).ok().unwrap();

    assert_eq!(&unit.name[..], "test.service");

    // Modify unit's desired state

    let modify_result = client.modify_unit("test.service", UnitStates::Loaded);

    assert!(modify_result.is_ok(), "{}", modify_result.err().unwrap());

    // List units

    let unit_pages = client.list_units(None).ok().unwrap();

    assert_eq!(unit_pages.units.len(), 1);

    let listed_unit = &unit_pages.units[0];

    assert_eq!(listed_unit.name, "test.service");

    // List unit states

    // for some reason GET /state sometimes returns no results even when there should be
    let unit_state_pages = retry(5, 500, || {
        client.list_unit_states(None, None, None).ok().unwrap()
    }, |unit_state_pages| {
        unit_state_pages.states.len() > 0 &&
            unit_state_pages.states[0].systemd_active_state == "inactive"
    }).ok().unwrap();

    let unit_state = &unit_state_pages.states[0];

    assert_eq!(unit_state.name, "test.service");
    assert_eq!(unit_state.machine_id, listed_unit.machine_id);
    assert_eq!(unit_state.systemd_load_state, "loaded");
    assert_eq!(unit_state.systemd_active_state, "inactive");
    assert_eq!(unit_state.systemd_sub_state, "dead");

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

#[test]
fn list_machines() {
    let client = Client::new("http://localhost:2999").unwrap();

    let machine_pages = client.list_machines(None).ok().unwrap();

    assert_eq!(machine_pages.machines.len(), 1);
}
