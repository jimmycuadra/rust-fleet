use rustc_serialize::json::Json;

use schema::UnitOption;

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
pub struct CreateUnit {
    pub desiredState: Json,
    pub options: Vec<UnitOption>,
}

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
pub struct ModifyUnit {
    pub desiredState: Json,
}
