pub enum UnitStates {
    Inactive,
    Loaded,
    Launched,
}

pub struct UnitOption {
    name: String,
    section: String,
    value: String,
}

pub struct Unit {
    current_state: UnitStates,
    desired_state: UnitStates,
    machine_id: String,
    name: String,
    options: Vec<UnitOption>,
}

pub struct UnitPage {
    units: Vec<Unit>,
    next_page_token: Option<String>,
}

pub struct UnitState {
    name: String,
    hash: String,
    machine_id: String,
    systemd_load_state: String,
    systemd_active_state: String,
    systemd_sub_state: String,
}

pub struct UnitStatePage {
    states: Vec<UnitState>,
    next_page_token: Option<String>,
}

#[cfg(test)]
mod unit_tests {
    use super::{Unit, UnitStates, UnitOption};

    #[test]
    fn it_can_be_constructed() {
        let unit_option = UnitOption {
            name: "Description".to_string(),
            section: "Unit".to_string(),
            value: "Example unit".to_string(),
        };

        Unit {
            current_state: UnitStates::Inactive,
            desired_state: UnitStates::Launched,
            machine_id: "abc123".to_string(),
            name: "example.service".to_string(),
            options: vec![unit_option],
        };
    }
}

#[cfg(test)]
mod unit_page_tests {
    use super::{Unit, UnitPage, UnitStates};

    #[test]
    fn it_can_be_paginated() {
        let unit = Unit {
            current_state: UnitStates::Inactive,
            desired_state: UnitStates::Launched,
            machine_id: "abc123".to_string(),
            name: "example.service".to_string(),
            options: vec![],
        };

        UnitPage {
            units: vec![unit],
            next_page_token: Some("8fefec2c".to_string()),
        };
    }

    #[test]
    fn it_can_have_no_additional_pages() {
        let unit = Unit {
            current_state: UnitStates::Inactive,
            desired_state: UnitStates::Launched,
            machine_id: "abc123".to_string(),
            name: "example.service".to_string(),
            options: vec![],
        };

        UnitPage {
            units: vec![unit],
            next_page_token: None,
        };
    }
}

#[cfg(test)]
mod unit_state_tests {
    use super::UnitState;

    #[test]
    fn it_can_be_constructed() {
        UnitState {
            name: "example.service".to_string(),
            hash: "abc123".to_string(),
            machine_id: "123abc".to_string(),
            systemd_load_state: "loaded".to_string(),
            systemd_active_state: "active".to_string(),
            systemd_sub_state: "running".to_string(),
        };
    }
}

#[cfg(test)]
mod unit_state_page_tests {
    use super::{UnitState, UnitStatePage};

    fn it_can_be_paginated() {
        let unit_state = UnitState {
            name: "example.service".to_string(),
            hash: "abc123".to_string(),
            machine_id: "123abc".to_string(),
            systemd_load_state: "loaded".to_string(),
            systemd_active_state: "active".to_string(),
            systemd_sub_state: "running".to_string(),
        };

        UnitStatePage {
            states: vec![unit_state],
            next_page_token: Some("8fefec2c".to_string()),
        };
    }

    fn it_can_have_no_additional_pages() {
        let unit_state = UnitState {
            name: "example.service".to_string(),
            hash: "abc123".to_string(),
            machine_id: "123abc".to_string(),
            systemd_load_state: "loaded".to_string(),
            systemd_active_state: "active".to_string(),
            systemd_sub_state: "running".to_string(),
        };

        UnitStatePage {
            states: vec![unit_state],
            next_page_token: None,
        };
    }
}
