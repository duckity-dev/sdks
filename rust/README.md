# Duckity Rust SDK

Welcome to the Duckity Rust SDK! It handles challenge issuances, solvings, and validations with the
duckling API.

This SDK does not include a wrapper for the management API. To use the management REST API, use
`reqwest` or another HTTP client instead.

## Installation

Add this crate to your project via

```sh
cargo add duckity
```

## Usage

> [!INFO]
> You need a Duckity application to follow this guide. If you don't have one yet, head over to
> duckity.dev and make one first.

Import `DuckityClient` into your code to start using the SDK.

```rust
use duckity::DuckityClient;
```

Then, initiate it like follows:

```rust
let client = DuckityClient::new();
```

This'll create a new instance of it pointing to `quack.duckity.dev` by default. If you're
self-hosting a duckling, you'll want to point the client to your own instance instead. To do so,
initialize the `DuckityClient` by calling `DuckityClient::with_domain()` instead.

```rust
let client = DuckityClient::with_domain("quack.example.com");
```

Note that this SDK does not support making requests over plain HTTP. You need an HTTPS server
for this to work properly.

### Getting and Solving a Challenge

To get a new challenge, use `DuckityClient::get_challenge()`. You'll need to have your application
ID and profile code around to do so.

```rust
const DUCKITY_APP_ID: &str = "my-duckity-app-id";
const DUCKITY_PROFILE_CODE: &str = "my-duckity-profile-code";

let client = DuckityClient::new();

let challenge = client.get_challenge(DUCKITY_APP_ID, DUCKITY_PROFILE_CODE).await?;
```

That's going to fetch a new challenge from the API using the specified app ID and profile code.

To solve such challenge, call `Challenge::solve()`. Note that this is CPU-intensive, so do not call
it from your async function directly. In tokio, for example, you should use
`tokio::task::spawn_blocking()` to solve the challenge.

```rust
let challenge = client.get_challenge(DUCKITY_APP_ID, DUCKITY_PROFILE_CODE).await?;

let token = tokio::task::spawn_blocking(move || {
    challenge.solve().encode()
}).await.unwrap();
```

Calling `Challenge::solve()` will perform the CPU computations needed to solve the challenge. It
returns an instance of `Solution` which contains the solution and a reference to the original
`Challenge`. `Solution::encode()` takes the challenge it solved and the solution and sticks them
together to create the base64-encoded solution token required by the validation endpoint.

A note on performance, **make sure to compile in release mode to see the real performance**. Since
solving challenges is CPU-bound, you may see massive performance differences between debug and
release builds (7.5s vs 300ms in our tests).

### Validating a Challenge

Now that you have such challenge, the server has to validate it. You must make the client
send over the solution token to the server in a real-world application (send the token string over
for that).

Taking our `String` solution token we just encoded in the example above, we now have to validate
it. That is done with `DuckityClient::validate_challenge()`. Its API is simple, `Ok` means the
solution was valid and `Err` means that either the solution wasn't valid or that it could not be
validated (HTTP errors, for example).

Let's suppose the following variables have their assigned values (they have to in a real-world
application):

```rust
let client_ip: IpAddr;
let solution_token: String;
let profile_code: String,
let app_id: String;
let app_secret: String;
```

Using those values, we can validate our solution token:

```rust
let client = DuckityClient::new();

let result = client.validate_challenge(
    app_id,
    app_secret,
    profile_code,
    solution_token,
    client_ip,
).await;

if result.is_ok() {
    // The token was valid!
} else {
    // The token was not valid, the request/action must be rejected.
}
```

That's it! A few notes before you go and keep building:

1. `profile_code` in the example above means the profile code you expect the challenge to have been
   issued for. A mismatching profile code will result in a rejected solution.
2. `client_ip` in the example above is the IP address of the client your server sees. Mismatching
   client IPs will result in a rejected solution.

## Examples

There's an example of the whole flow implemented at `rust/examples/challenge.rs` in the SDKs
repository. To run it, clone the repository with `git clone https://github.com/duckity-dev/sdks`
and CD into the `rust/` directory. You can run it with the following command:

```sh
cargo run --example challenge --release
```

You'll be guided on how to set it up. It's easy, so good luck!

## Contributing

Contributions of any kind are welcome! Suggestions, issues, PRs, and everything else goes into our
[SDKs repository in GitHub](https://github.com/duckity-dev/sdks). We reward good contributions with
Duckity credits 😉
