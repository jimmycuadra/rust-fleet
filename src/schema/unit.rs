use rustc_serialize::json::{Json, ToJson};

/// The possible runtime states a unit can be in.
pub enum UnitStates {
    /// The unit has not been loaded onto a machine and is not running.
    Inactive,
    /// The unit has been loaded onto a machine but is not running.
    Loaded,
    /// The unit has been loaded onto a machine and is running.
    Launched,
}

impl UnitStates {
    /// Returns the `UnitStates` variant corresponding to a string representation.
    ///
    /// # Panics
    ///
    /// Panics if the string representation provided does not match a valid variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fleet::UnitStates;
    /// UnitStates::from_str("launched"); // UnitStates::Launched
    /// ```
    pub fn from_str(s: &str) -> UnitStates {
        match s {
            "inactive" => UnitStates::Inactive,
            "loaded" => UnitStates::Loaded,
            "launched" => UnitStates::Launched,
            variant => panic!("not a valid UnitStates variant: {}", variant),
        }
    }
}

impl ToJson for UnitStates {
    fn to_json(&self) -> Json {
        let value = match *self {
            UnitStates::Inactive => "inactive",
            UnitStates::Loaded => "loaded",
            UnitStates::Launched => "launched",
        };

        Json::String(value.to_string())
    }
}

/// A single line from a unit file. Unit files consist of key/value pairs divided into sections.
#[derive(RustcEncodable)]
pub struct UnitOption {
    /// The key.
    pub name: String,
    /// The section the key/value pair will appear under.
    pub section: String,
    /// The value.
    pub value: String,
}

/// A single fleet unit, which is a systemd unit with optional fleet-specific data.
pub struct Unit {
    /// The unit's state.
    pub current_state: UnitStates,
    /// The unit's future state. Eventually fleetd will move the unit into this state, but it might
    /// not have happened yet.
    pub desired_state: UnitStates,
    /// The unique ID of the machine where the unit is loaded/running, unless it is inactive.
    pub machine_id: Option<String>,
    /// The unit's name.
    pub name: String,
    /// The lines of key/value pairs that make up a unit file.
    pub options: Vec<UnitOption>,
}

/// A single page from a paginated collection of units.
pub struct UnitPage {
    /// The units in this page.
    pub units: Vec<Unit>,
    /// If `Some`, at least one additional page is available and can be requested with this token.
    pub next_page_token: Option<String>,
}

/// The current state of a unit.
pub struct UnitState {
    /// The unit's name.
    pub name: String,
    /// A unique hash for the unit.
    pub hash: String,
    /// The unique ID of the machine where the unit is loaded/running, unless it is inactive.
    pub machine_id: Option<String>,
    /// The load state of the unit as reported by systemd.
    pub systemd_load_state: String,
    /// The active state of the unit as reported by systemd.
    pub systemd_active_state: String,
    /// The sub state of the unit as reported by systemd.
    pub systemd_sub_state: String,
}

/// A single page from a paginated collection of unit states.
pub struct UnitStatePage {
    /// The unit states in this page.
    pub states: Vec<UnitState>,
    /// If `Some`, at least one additional page is available and can be requested with this token.
    pub next_page_token: Option<String>,
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
            machine_id: Some("abc123".to_string()),
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
            machine_id: Some("abc123".to_string()),
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
            machine_id: Some("abc123".to_string()),
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
            machine_id: Some("123abc".to_string()),
            systemd_load_state: "loaded".to_string(),
            systemd_active_state: "active".to_string(),
            systemd_sub_state: "running".to_string(),
        };
    }
}

#[cfg(test)]
mod unit_state_page_tests {
    use super::{UnitState, UnitStatePage};

    #[test]
    fn it_can_be_paginated() {
        let unit_state = UnitState {
            name: "example.service".to_string(),
            hash: "abc123".to_string(),
            machine_id: Some("123abc".to_string()),
            systemd_load_state: "loaded".to_string(),
            systemd_active_state: "active".to_string(),
            systemd_sub_state: "running".to_string(),
        };

        UnitStatePage {
            states: vec![unit_state],
            next_page_token: Some("8fefec2c".to_string()),
        };
    }

    #[test]
    fn it_can_have_no_additional_pages() {
        let unit_state = UnitState {
            name: "example.service".to_string(),
            hash: "abc123".to_string(),
            machine_id: Some("123abc".to_string()),
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
