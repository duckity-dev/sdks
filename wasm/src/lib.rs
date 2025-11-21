#![doc = include_str!("../README.md")]

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use gloo_net::http::Request;
use num_bigint::BigUint;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

/// The size of a Duckity challenge in bytes.
const CHALLENGE_SIZE: usize = 397;

/// A client for interacting with the Duckity API.
///
/// To create a new client, use `DuckityClient.new()`. If you're using a self-hosted duckling,
/// use `DuckityClient.with_domain()` to point to your custom domain.
///
/// To get a challenge, use `DuckityClient.get_challenge()`. To solve it, use
/// `Challenge.solve()`. Use `Solution.encode()` to get the encoded solution string.
#[wasm_bindgen]
pub struct DuckityClient {
    /// The domain the client is pointing to.
    domain: String,
}

#[wasm_bindgen]
impl DuckityClient {
    /// Create a new Duckity client.
    ///
    /// Use `DuckityClient.with_domain()` instead if you want to point to a custom domain.
    ///
    /// Returns:
    /// `DuckityClient` - A new Duckity client.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            domain: "quack.duckity.dev".to_string(),
        }
    }

    /// Create a new Duckity client pointing to a custom domain.
    ///
    /// Use this if you're self-hosting Duckity or using a different environment.
    ///
    /// Arguments:
    /// * `domain` - The domain to point the client to.
    ///
    /// Returns:
    /// `DuckityClient` - A new Duckity client.
    #[wasm_bindgen]
    pub fn with_domain(domain: String) -> Self {
        Self { domain }
    }

    /// Get a challenge for the given application ID and profile code.
    ///
    /// Arguments:
    /// * `app_id` - The application ID to get the challenge for.
    /// * `profile_code` - The profile code to use for the challenge.
    ///
    /// Returns:
    /// * `Challenge` - The solution token, if successful.
    /// * `String` - An error string if the request failed.
    #[wasm_bindgen]
    pub async fn get_challenge(
        &self,
        app_id: String,
        profile_code: String,
    ) -> Result<Challenge, String> {
        let payload = ChallengeRequestPayload {
            profile: profile_code.to_string(),
        };

        let response = Request::post(&format!("https://{}/v1/challenges/{}", self.domain, app_id))
            .json(&payload)
            .unwrap()
            .send()
            .await
            .err_to_string()?;

        if response.status() == 200 {
            let bytes = response.binary().await.err_to_string()?;

            let challenge = Challenge::decode(&bytes).err_to_string()?;

            Ok(challenge)
        } else {
            let error_response: ErrorResponse = response.json().await.err_to_string()?;

            Err(DuckityError::ApiError(
                error_response.title,
                error_response.message,
            ))
            .err_to_string()
        }
    }
}

impl Default for DuckityClient {
    fn default() -> Self {
        Self::new()
    }
}

trait DuckityResultToString<T> {
    fn err_to_string(self) -> Result<T, String>;
}

impl<T, E> DuckityResultToString<T> for Result<T, E>
where
    E: Into<DuckityError>,
{
    fn err_to_string(self) -> Result<T, String> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(error.into().to_string()),
        }
    }
}

/// An error that can occur when using the Duckity client.

#[derive(Debug, thiserror::Error)]
pub enum DuckityError {
    /// An error occurred with the Duckity client while making an HTTP request.
    #[error("An error occurred with the Duckity client while making an HTTP request: {0}")]
    RequestFailed(#[from] gloo_net::Error),

    /// An error occurred while decoding the challenge.
    #[error(
        "An error occurred while decoding the challenge. Did the API return a valid response? {0}"
    )]
    DecodingFailed(&'static str),

    /// An API error occurred.
    #[error("An API error occurred: {0}: {1}")]
    ApiError(String, String),
}

#[derive(Serialize)]
struct ChallengeRequestPayload {
    /// The profile code to use for the challenge.
    profile: String,
}

/// A Duckity challenge.
///
/// Use `Challenge.solve()` to solve the challenge and get a `Solution`.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Challenge(Vec<u8>);

#[wasm_bindgen]
impl Challenge {
    /// Decode a Duckity challenge from bytes.
    ///
    /// Arguments:
    /// * `data` - The bytes to decode the challenge from.
    ///
    /// Returns:
    /// * [`Ok<Challenge>`] - The decoded challenge.
    /// * [`Err<DuckityError>`] - An error if the challenge was invalid.
    fn decode(data: &[u8]) -> Result<Self, DuckityError> {
        if data.len() != CHALLENGE_SIZE {
            return Err(DuckityError::DecodingFailed(
                "The challenge size in bytes was not the expected byte size.",
            ));
        }

        Ok(Self(data.to_vec()))
    }

    /// Get the 'x' value from the challenge.
    ///
    /// Returns:
    /// * [`BigUint`] - The 'x' value.
    fn x(&self) -> BigUint {
        BigUint::from_bytes_be(&self.0[32..64])
    }

    /// Get the 'p' value from the challenge.
    ///
    /// Returns:
    /// * [`BigUint`] - The 'p' value.
    fn p(&self) -> BigUint {
        BigUint::from_bytes_be(&self.0[64..320])
    }

    /// Get the 't' value from the challenge.
    ///
    /// Returns:
    /// * [`u32`] - The 't' value.
    fn t(&self) -> u32 {
        u32::from_be_bytes(self.0[320..324].try_into().unwrap())
    }

    /// Solve the Duckity challenge.
    ///
    /// Note that this operation is computationally-intensive. Make sure to run it from a worker
    /// not to block the main thread.
    ///
    /// Returns:
    /// * `Solution` - The solution to the challenge.
    #[wasm_bindgen]
    pub fn solve(&self) -> Solution {
        let x = self.x();
        let p = self.p();
        let t = self.t();

        let mut y = x;
        for _ in 0..t {
            let e = (&p + (BigUint::ZERO + 1u8)) >> 2; // (p+1)/4
            y = y.modpow(&e, &p);
        }

        Solution(self.clone(), y)
    }
}

/// The solution to a Duckity challenge.
#[wasm_bindgen]
pub struct Solution(Challenge, BigUint);

#[wasm_bindgen]
impl Solution {
    /// Encode the solution as a base64 URL-safe string.
    ///
    /// Returns:
    /// * `String` - The encoded solution.
    #[wasm_bindgen]
    pub fn encode(&self) -> String {
        let mut buf = Vec::with_capacity(CHALLENGE_SIZE + 256);

        buf.extend_from_slice(&self.0.0);
        buf.extend_from_slice(&self.1.to_bytes_be());

        BASE64_URL_SAFE_NO_PAD.encode(buf)
    }

    /// Get the raw size of the solution in bytes.
    ///
    /// Returns:
    /// * `Number` - The size of the solution in bytes.
    #[wasm_bindgen]
    pub fn raw_size(&self) -> usize {
        self.0.0.len() + self.1.to_bytes_be().len()
    }
}

#[derive(serde::Deserialize)]
struct ErrorResponse {
    title: String,
    message: String,
}
