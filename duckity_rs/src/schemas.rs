use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChallengeRequest {
    pub application_id: String,
    pub protection_profile_id: String,

    /// CCTC key-value pairs.
    pub keys: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ChallengeResponse {
    /// The encoded challenge string.
    pub challenge: String,
}
