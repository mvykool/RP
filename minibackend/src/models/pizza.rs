// bring serialize, and deserialize from serde, bring validator as well
// add derive macro, with validate, deserialize, serialize
// create a public structure for the request, add a validation to check:
// length, min one characther, else a message, return public string
use serde::{Deserialize, Serialize};
use validator::Validate;

//macro here
#[derive(Validate, Deserialize, Serialize)]
pub struct BuyRequest {
    #[validate(length(min = 1, message = "pizza required"))]
    pub pizza_name: String,
}
