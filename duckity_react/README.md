# @duckity/react

A React wrapper for [Duckity](https://duckity.dev/).

# Installation

```bash
# Using npm, bun, yarn, or pnpm
npm install @duckity/react
bun add @duckity/react
yarn add @duckity/react
pnpm add @duckity/react
```

# Quick Start

The following is a quick example of how to use the `useChallenge` hook in a React component.

```tsx
import { SubmitEvent } from "react";
import { useChallenge } from "@duckity/react";
import { login } from "@/lib/api";

function App() {
  const [isLoading, setIsLoading] = useState(false);
  const [username, setUsername] = useState("");
  const duckity = useChallenge(
    process.env.NEXT_PUBLIC_DUCKITY_PROTECTION_PROFILE_ID,
  );

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsLoading(true);

    let solution = await duckity.solve({
      keys: {
        username,
      },
    });

    // Send the solution to your backend for verification and login.
    let response = await login({
      username,
      token: solution,
    });

    setIsLoading(false);
  }

  return (
    <form onSubmit={handleSubmit}>
      <input
        type="text"
        placeholder="username"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        disabled={isLoading}
      />
      <button type="submit" disabled={isLoading}>
        {isLoading
          ? duckity.isLoading
            ? "Verifying you're a human..."
            : "Logging in..."
          : "Login"}
      </button>
    </form>
  );
}
```

In the example above, when the form is submitted we call `duckity.solve()` to fetch and solve a
challenge. The optional `keys` parameter we passed are Custom-Context Threat Correlation keys,
which you must configure from the Duckity dashboard before using.

The solution token will both be available from `useChallenge().solution` and the return value of
`duckity.solve()`.

The challenge is solved in a web worker, so the UI will remain responsive while the challenge is
being solved. You can check `isIdle`, `isLoading`, and `isError` to update the UI accordingly, or
use `status` for a more granular state of the challenge.

When to call `duckity.solve()` depends on your use case. In the example above, we call it when the
user submits the login form because we need their username as a CCTC key. However, if you don't
need to pass any CCTC keys, you can call `duckity.solve()` as soon as the component mounts to
pre-solve a challenge and save the user loading time later on. For example:

```tsx
import { useEffect } from "react";
import { useChallenge } from "@duckity/react";
import { action } from "@/lib/api";

function App() {
  const duckity = useChallenge(
    process.env.NEXT_PUBLIC_DUCKITY_PROTECTION_PROFILE_ID,
  );

  useEffect(() => {
    if (!duckity.solution && duckity.isIdle) {
      duckity.solve();
    }
  }, [duckity.status, duckity.solution]);

  async function handleClick() {
    // Wait until a solution is available if it's not already.
    let solution = await duckity.wait();

    let response = await action({
      token: solution,
    });

    // Solutions can only be used once, so invalidate the current solution to get a new one for the
    // next action.
    duckity.invalidate();
  }

  return (
    <button onClick={handleClick} disabled={duckity.isLoading}>
      {duckity.isLoading ? "Solving challenge..." : "Do Action"}
    </button>
  );
}
```

In the example above, we call `duckity.solve()` as soon as the component mounts to pre-solve a
challenge. Then, when the user clicks the button, we call `duckity.wait()` to wait for the solution
to be available if it's not already. After the action is performed, we call `duckity.invalidate()`
to drop the current solution. Our `useEffect` will cause a new one will be fetched and solved for
the next action.
