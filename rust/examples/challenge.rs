//! An example demonstrating how to use Duckity to fetch, solve, and validate a challenge.
//!
//! Before running this example, make sure to set the `DUCKITY_APP_ID`, `DUCKITY_APP_SECRET`,
//! and `DUCKITY_PROFILE_CODE` constants with your Duckity application credentials.
//!
//! Keep in mind that for the purpose of the example showing the full flow, we're putting the
//! verification and the app secret in the client. In a real-world application, the verification
//! should be done on the server side to keep the app secret secure.
//!
//! To run this example, use the following command:
//!
//! ```sh
//! cargo run --example challenge --release
//! ```

use colored::Colorize;
use duckity::DuckityClient;
use tokio::time::Instant;

const DUCKITY_APP_ID: &str = "";
const DUCKITY_APP_SECRET: &str = "";
const DUCKITY_PROFILE_CODE: &str = "";
const DUCKITY_DOMAIN: &str = "quack.duckity.dev";

#[allow(clippy::const_is_empty)]
#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        println!("{}", "------\nWARNING: You are running this example in debug mode. This is tens of times slower than release mode. For the real performance, compile in release mode with `cargo run --release --example challenge`.\n------".bright_yellow());
        println!();
    }

    println!("Welcome to the Duckity challenge example!");
    println!();

    if DUCKITY_APP_ID.is_empty() || DUCKITY_APP_SECRET.is_empty() || DUCKITY_PROFILE_CODE.is_empty()
    {
        eprintln!(
            "{}",
            "------\nSeems like it's your first time here! Open rust/examples/challenge.rs and update the DUCKITY_APP_ID, DUCKITY_APP_SECRET, and DUCKITY_PROFILE_CODE constants at the top of the file. This'll work once that's done!\n------".bright_yellow()
        );
        return;
    }

    println!("Domain: {}", DUCKITY_DOMAIN);
    println!("App ID: {}", DUCKITY_APP_ID);
    println!("App Secret: {}", "*".repeat(DUCKITY_APP_SECRET.len()));
    println!("Profile Code: {}", DUCKITY_PROFILE_CODE);
    println!();
    println!("Fetching a challenge from the server...");

    let client = DuckityClient::with_domain(DUCKITY_DOMAIN);

    let challenge = client
        .get_challenge(DUCKITY_APP_ID, DUCKITY_PROFILE_CODE)
        .await
        .expect("The challenge couldn't be fetched!");

    println!("Solving the challenge (hardness {})...", challenge.t());

    let start = Instant::now();
    // We're not using tokio::task::spawn_blocking here because the example is synchronous. In a
    // real-world application with background tasks, use tokio::task::spawn_blocking() to avoid
    // blocking the async runtime.
    let solution = challenge.solve();
    let elapsed = start.elapsed();

    println!(
        "Solved! The solution is {} bytes long.",
        solution.raw_size()
    );

    println!("Validating the solution with the server...");

    let ip = challenge.ip().unwrap();

    let response = client
        .validate_challenge(
            DUCKITY_APP_ID,
            DUCKITY_APP_SECRET,
            DUCKITY_PROFILE_CODE,
            solution.encode(),
            ip,
        )
        .await;

    if let Err(error) = response {
        eprintln!("{}", format!("Validation failed: {}", error).bright_red());
        return;
    } else {
        println!("{}", "Validation succeeded!".bright_green());
    }

    println!();
    println!("Solved the challenge in {:.2?}!", elapsed);
}
