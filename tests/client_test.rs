extern crate fleet;

use std::thread::sleep_ms;

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
    for try in 0..5 {
        let unit = client.get_unit("test.service").ok().unwrap();

        assert_eq!(&unit.name[..], "test.service");

        match unit.machine_id {
            Some(_) => break,
            None => {
                if try == 4 {
                    panic!("test.service never launched");
                } else {
                    sleep_ms(500);
                }
            }
        }
    }

    // Modify unit's desired state

    let modify_result = client.modify_unit("test.service", UnitStates::Loaded);

    assert!(modify_result.is_ok(), "{}", modify_result.err().unwrap());

    // List units

    let units = client.list_units().ok().unwrap();

    assert_eq!(units.len(), 1);

    let listed_unit = &units[0];

    assert_eq!(listed_unit.name, "test.service");

    // List unit states

    // for some reason GET /state sometimes returns no results even when there should be
    for try in 0..5 {
        let unit_states = client.list_unit_states(None, None).ok().unwrap();

        if unit_states.len() > 0 {
            let unit_state = &unit_states[0];

            assert_eq!(unit_state.name, "test.service");
            assert_eq!(unit_state.machine_id, listed_unit.machine_id);
            assert_eq!(unit_state.systemd_load_state, "loaded");
            assert_eq!(unit_state.systemd_active_state, "inactive");
            assert_eq!(unit_state.systemd_sub_state, "dead");
        } else if try == 4{
            panic!("no unit states were returned");
        } else {
            sleep_ms(500);
        }
    }

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
