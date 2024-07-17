use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BadRequestError<'a> {
    pub message: &'a str,
    pub error: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
}
