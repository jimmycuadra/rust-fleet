use std::collections::HashMap;

pub struct Machine {
    id: String,
    metadata: Option<HashMap<String, String>>,
    primary_ip: String,
}

pub struct MachinePage {
    machines: Vec<Machine>,
    next_page_token: Option<String>,
}

#[cfg(test)]
mod machine_tests {
    use std::collections::HashMap;

    use super::Machine;

    #[test]
    fn it_can_be_constructed() {
        Machine {
            id: "abc123".to_string(),
            metadata: None,
            primary_ip: "1.2.3.4".to_string(),
        };
    }

    #[test]
    fn it_can_be_constructed_with_metdata() {
        let mut metadata = HashMap::new();

        metadata.insert("region".to_string(), "us-east-1".to_string());

        Machine {
            id: "abc123".to_string(),
            metadata: Some(metadata),
            primary_ip: "1.2.3.4".to_string(),
        };
    }
}

#[cfg(test)]
mod machine_page_tests {
    use super::{Machine,MachinePage};

    #[test]
    fn it_can_be_paginated() {
        let machine = Machine {
            id: "abc123".to_string(),
            metadata: None,
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
            metadata: None,
            primary_ip: "1.2.3.4".to_string(),
        };

        MachinePage {
            machines: vec![machine],
            next_page_token: None,
        };
    }
}
