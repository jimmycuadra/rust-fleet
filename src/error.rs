use std::error::Error;
use std::fmt;

use hyper::client::Response;
use hyper::error::HttpError;
use rustc_serialize::json::Json;

pub struct FleetError {
    code: Option<u16>,
    message: Option<String>,
}

impl FleetError {
    pub fn from_hyper_error(error: &HttpError) -> FleetError {
        FleetError {
            code: None,
            message: Some(error.description().to_string()),
        }
    }

    pub fn from_hyper_response(response: &mut Response) -> FleetError {
        FleetError {
            code: Some(response.status.to_u16()),
            message: extract_message(response),
        }
    }
}

impl fmt::Display for FleetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.code.unwrap_or(0),
            self.message.clone().unwrap_or("Unknown error".to_string()),
       )
    }
}

fn extract_message(response: &mut Response) -> Option<String> {
   match Json::from_reader(response) {
       Ok(json) => {
           match json.find_path(&["error", "message"]) {
               Some(message_json) => match message_json.as_string() {
                   Some(message) => {
                       if message.len() == 0 {
                           Some("Error in JSON response from Fleet was empty".to_string())
                       } else {
                           Some(message.to_string())
                       }
                   },
                   None => Some("Error in JSON response from Fleet was empty".to_string()),
               },
               None => Some("Error parsing JSON response from Fleet".to_string()),
           }
       },
       Err(error) => Some(error.description().to_string()),
   }
}
