use std::collections::HashMap;

/// A host node running fleetd.
pub struct Machine {
    /// The machine's unique ID.
    pub id: String,
    /// Arbitrary fleet metadata associated with the machine.
    pub metadata: HashMap<String, String>,
    /// The machine's IP address.
    pub primary_ip: String,
}

/// A single page from a paginated collection of machines.
pub struct MachinePage {
    /// The machines in this page.
    pub machines: Vec<Machine>,
    /// If `Some`, at least one additional page is available and can be requested with this token.
    pub next_page_token: Option<String>,
}

#[cfg(test)]
mod machine_tests {
    use std::collections::HashMap;

    use super::Machine;

    #[test]
    fn it_can_be_constructed() {
        Machine {
            id: "abc123".to_string(),
            metadata: HashMap::new(),
            primary_ip: "1.2.3.4".to_string(),
        };
    }

    #[test]
    fn it_can_be_constructed_with_metdata() {
        let mut metadata = HashMap::new();

        metadata.insert("region".to_string(), "us-east-1".to_string());

        Machine {
            id: "abc123".to_string(),
            metadata: metadata,
            primary_ip: "1.2.3.4".to_string(),
        };
    }
}

#[cfg(test)]
mod machine_page_tests {
    use std::collections::HashMap;

    use super::{Machine,MachinePage};

    #[test]
    fn it_can_be_paginated() {
        let machine = Machine {
            id: "abc123".to_string(),
            metadata: HashMap::new(),
            primary_ip: "1.2.3.4".to_string(),
        };

        MachinePage {
            machines: vec![machine],
            next_page_token: Some("8fefec2c".to_string()),
        };
    }

    #[test]
    fn it_can_have_no_additional_pages() {
        let machine = Machine {
            id: "abc123".to_string(),
            metadata: HashMap::new(),
            primary_ip: "1.2.3.4".to_string(),
        };

        MachinePage {
            machines: vec![machine],
            next_page_token: None,
        };
    }
}
