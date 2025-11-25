# Duckity Python SDK

Welcome to the Duckity Python SDK! This SDK handles fetching, solving, and validating Duckity
challenges. The solving backend is built in Rust, so you can solve challenges from Python at native
speed.

## Installation

To install this SDK, run the following command in your terminal:

```sh
pip install duckity
```

Otherwise, if you're using another package manager, use that PM's command. For example, with `uv`,
run:

```sh
uv add duckity
```

## Usage

There's a single interface through which you'll interact with the duckling API.

```py
from duckity import DuckityClient

client = DuckityClient()
```

That will create a new `DuckityClient` instance pointing at `quack.duckity.dev`. If you self-host a
duckling, create a `DuckityClient` with a custom domain name instead:

```py
from duckity import DuckityClient

client = DuckityClient("quack.example.com")
```

Every method of `DuckityClient` has an alternative asynchronous method prefixed with `a`. For
example, `fetch()` and `afetch()`, or `solve()` and `asolve()`.

### Solving Challenges

To solve a challenge, you need to fetch it first.

```py
from duckity import Challenge

challenge: Challenge = client.fetch("app-id", "profile-code")
```

Once you have a challenge, solve it by calling `solve()` on it.

```py
solution: str = challenge.solve()
```

The solution will be an already-encoded token ready to send to the server.

As stated before, every method has an asynchronous alternative. In asynchronous environments, do
not use the synchronous methods as they will block your async runtime. `Challenge.solve()` is
notable here, since it is CPU-intensive. `Challenge.asolve()` wraps `Challenge.solve()` in
`asyncio.to_thread()` to prevent it from blocking the event loop.

### Validating Challenges

To validate challenges, use `DuckityClient.validate()` (or its asynchronous alternative
`DuckityClient.avalidate()`).

```py
is_valid: bool = client.validate(
    "app-id",
    "app-secret",
    "profile-code",
    "client-ip",
    "solution-token"
)

if not is_valid:
    print("The challenge was not valid!")
    return

# Do your stuff here
```

## Examples

There's an example of solving a challenge implemented at `python/examples/challenge.py` in the SDKs
repository. To run it, clone the repository with `git clone https://github.com/duckity-dev/sdks`
and CD into the `python/` directory. You can run it with the following command:

```sh
# The example works as a singletone file if you install the dependencies in pyproject.toml
# manually.
uv sync
uv run examples/challenge.py
```

You'll be guided on how to set it up. It's easy, so good luck!

## Contributing

Contributions of any kind are welcome! Suggestions, issues, PRs, and everything else goes into our
[SDKs repository in GitHub](https://github.com/duckity-dev/sdks). We reward good contributions with
Duckity credits 😉
