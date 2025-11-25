#![doc = include_str!("../README.md")]

use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    time::Duration,
};

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use num_bigint::BigUint;
use reqwest::Client;
use serde::Serialize;

/// The size of a Duckity challenge in bytes.
const CHALLENGE_SIZE: usize = 397;

/// A client for interacting with the Duckity API.
///
/// To create a new client, use [`DuckityClient::new()`]. If you're using a self-hosted duckling,
/// use [`DuckityClient::with_domain()`] to point to your custom domain.
///
/// To get a challenge, use [`DuckityClient::get_challenge()`]. To solve it, use
/// [`Challenge::solve()`]. Use [`Solution::encode()`] to get the encoded solution string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuckityClient {
    /// The domain the client is pointing to.
    domain: String,
}

impl DuckityClient {
    /// Create a new Duckity client.
    ///
    /// Use [`DuckityClient::with_domain()`] instead if you want to point to a custom domain.
    ///
    /// Returns:
    /// [`DuckityClient`] - A new Duckity client.
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
    /// [`DuckityClient`] - A new Duckity client.
    pub fn with_domain(domain: impl ToString) -> Self {
        Self {
            domain: domain.to_string(),
        }
    }

    /// Get a challenge for the given application ID and profile code.
    ///
    /// Arguments:
    /// * `app_id` - The application ID to get the challenge for.
    /// * `profile_code` - The profile code to use for the challenge.
    ///
    /// Returns:
    /// * [`Ok<Challenge>`] - The challenge if successful.
    /// * [`Err<DuckityError>`] - An error if the request failed.
    pub async fn get_challenge(
        &self,
        app_id: impl ToString,
        profile_code: impl ToString,
    ) -> Result<Challenge, DuckityError> {
        let payload = ChallengeRequestPayload {
            profile: profile_code.to_string(),
        };

        let response = Client::new()
            .post(format!(
                "https://{}/v1/challenges/{}",
                self.domain,
                app_id.to_string()
            ))
            .json(&payload)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if response.status().is_success() {
            let bytes = response.bytes().await?;

            let challenge = Challenge::decode(&bytes)?;

            Ok(challenge)
        } else {
            let error_response: ErrorResponse = response.json().await?;

            Err(DuckityError::ApiError(
                error_response.title,
                error_response.message,
            ))
        }
    }

    /// Validate a challenge solution with the server.
    ///
    /// Arguments:
    /// * `app_id` - The application ID the challenge was issued for.
    /// * `app_secret` - The application secret for authentication.
    /// * `profile_code` - The profile code used for the challenge.
    /// * `solution` - The solution to validate, as a base64 URL-safe encoded string.
    /// * `client_ip` - The client IP address the challenge was issued for.
    ///
    /// Returns:
    /// * [`Ok<()>`] - If the validation was successful.
    /// * [`Err<DuckityError>`] - An error if the validation failed.
    pub async fn validate_challenge(
        &self,
        app_id: impl ToString,
        app_secret: impl ToString,
        profile_code: impl ToString,
        solution: String,
        client_ip: IpAddr,
    ) -> Result<(), DuckityError> {
        let payload = ValidationRequest {
            token: solution,
            ip: client_ip,
            profile: profile_code.to_string(),
        };

        let response = Client::new()
            .post(format!(
                "https://{}/v1/challenges/{}/validate",
                self.domain,
                app_id.to_string()
            ))
            .json(&payload)
            .timeout(Duration::from_secs(10))
            .bearer_auth(app_secret.to_string())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_response: ErrorResponse = response.json().await?;

            Err(DuckityError::ApiError(
                error_response.title,
                error_response.message,
            ))
        }
    }
}

impl Default for DuckityClient {
    fn default() -> Self {
        Self::new()
    }
}

/// An error that can occur when using the Duckity client.
#[derive(Debug, thiserror::Error)]
pub enum DuckityError {
    /// An error occurred with the Duckity client while making an HTTP request.
    #[error("An error occurred with the Duckity client while making an HTTP request: {0}")]
    RequestFailed(#[from] reqwest::Error),

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
/// Use [`Challenge::solve()`] to solve the challenge and get a [`Solution`].
pub struct Challenge(Vec<u8>);

impl Challenge {
    /// Decode a Duckity challenge from bytes.
    ///
    /// Arguments:
    /// * `data` - The bytes to decode the challenge from.
    ///
    /// Returns:
    /// * [`Ok<Challenge>`] - The decoded challenge.
    /// * [`Err<DuckityError>`] - An error if the challenge was invalid.
    pub fn decode(data: &[u8]) -> Result<Self, DuckityError> {
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
    pub fn x(&self) -> BigUint {
        BigUint::from_bytes_be(&self.0[32..64])
    }

    /// Get the 'p' value from the challenge.
    ///
    /// Returns:
    /// * [`BigUint`] - The 'p' value.
    pub fn p(&self) -> BigUint {
        BigUint::from_bytes_be(&self.0[64..320])
    }

    /// Get the 't' value from the challenge.
    ///
    /// Returns:
    /// * [`u32`] - The 't' value.
    pub fn t(&self) -> u32 {
        u32::from_be_bytes(self.0[320..324].try_into().unwrap())
    }

    /// Get the client IP address the challenge was issued for.
    /// 
    /// Returns:
    /// * [`Ok<IpAddr>`] - The client IP address.
    /// * [`Err<DuckityError>`] - An error if the IP address could not be decoded.
    pub fn ip(&self) -> Result<IpAddr, DuckityError> {
        let client_ip_bytes = &self.0[340..357];

        match client_ip_bytes[0] {
            4 => {
                let octets: [u8; 4] = client_ip_bytes[1..5].try_into().expect("The slice had an incorrect length for challenge's IPv4 bytes (expected 4 bytes, but it wasn't 4 bytes)");
                Ok(IpAddr::V4(Ipv4Addr::from(octets)))
            }
            6 => {
                let octets: [u8; 16] = client_ip_bytes[1..17].try_into().expect("The slice had an incorrect length for challenge's IPv6 bytes (expected 16 bytes, but it wasn't 16 bytes)");
                Ok(IpAddr::V6(Ipv6Addr::from(octets)))
            }
            _ => Err(DuckityError::DecodingFailed(
                "The challenge contained an invalid IP address version. Only IPv4 and IPv6 are supported.",
            )),
        }
    }

    /// Solve the Duckity challenge.
    ///
    /// Note that this operation can be computationally intensive depending on the hardness 't'. Do
    /// not run this on the main thread in a GUI application, nor in an async function. To run it
    /// in tokio, for example, use `tokio::task::spawn_blocking`.
    ///
    /// Returns:
    /// * [`Solution<'_>`] - The solution to the challenge.
    pub fn solve(&self) -> Solution<'_> {
        let x = self.x();
        let p = self.p();
        let t = self.t();

        let mut y = x;
        for _ in 0..t {
            let e = (&p + (BigUint::ZERO + 1u8)) >> 2; // (p+1)/4
            y = y.modpow(&e, &p);
        }

        Solution(self, y)
    }
}

/// The solution to a Duckity challenge.
pub struct Solution<'a>(&'a Challenge, BigUint);

impl Solution<'_> {
    /// Encode the solution as a base64 URL-safe string.
    ///
    /// Returns:
    /// * [`String`] - The encoded solution.
    pub fn encode(&self) -> String {
        let mut buf = Vec::with_capacity(CHALLENGE_SIZE + 256);

        buf.extend_from_slice(&self.0.0);
        buf.extend_from_slice(&self.1.to_bytes_be());

        BASE64_URL_SAFE_NO_PAD.encode(buf)
    }

    /// Get the raw size of the solution in bytes.
    ///
    /// Returns:
    /// * [`usize`] - The size of the solution in bytes.
    pub fn raw_size(&self) -> usize {
        self.0.0.len() + self.1.to_bytes_be().len()
    }
}

#[derive(serde::Serialize)]
struct ValidationRequest {
    token: String,
    ip: IpAddr,
    profile: String,
}

#[derive(serde::Deserialize)]
struct ErrorResponse {
    title: String,
    message: String,
}
