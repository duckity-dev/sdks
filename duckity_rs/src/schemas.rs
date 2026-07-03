use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChallengeRequest {
    /// The protection profile's ID.
    pub id: String,

    /// CCTC key-value pairs.
    pub keys: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ChallengeResponse {
    /// The encoded challenge string.
    pub challenge: String,
}
