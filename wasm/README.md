# Duckity WASM SDK

Welcome to the Duckity WASM SDK! This SDK implements Duckity's challenge solving for websites and
web apps.

It's a port of Duckity's Rust SDK.

## Installation

To install the Duckity WASM SDK, run this command in your command line:

```sh
bun add @duckity/wasm
```

Replace `bun` with `npm`, `pnpm`, `yarn`, or your preferred package manager in case `bun` is not
your choice.

## Usage

To use the Ducktiy SDK, import `DuckityClient` into your code. That's all you'll need.

```ts
import DuckityClient from "@duckity/wasm";

let client = new DuckityClient();
```

Optionally, if you're self-hosting a duckling, pass the domain name as a parameter to the
constructor, like follows:

```ts
import DuckityClient from "@duckity/wasm";

let client = new DuckityClient("quack.example.com");
```

By default, it points to Duckity's hosted duckling, `quack.duckity.dev`.

Solving a challenge is quite simple. Just call `DuckityClient.solve()` with the application ID and
profile code and that's it! You'll be given a solution token as a string.

```ts
let solution: string = client.solve("app-id", "profile-code");
```

## jsDelivr and CDNs

jsDelivr and other CDNs will not work with this SDK by default because solving challenges is done
in a worker, which gets blocked because of not being same-origin. To use this SDK on bare HTML
files or other non-npm environments, visit this repository and download the files under
`/wasm/js/dist`. Serve them as static files.

For example, given the following folder structure:

```
www/
  index.html
  js/
    duckity/
      duckity.js
      worker.js
      wasm/
        duckity.js
        duckity_bg.wasm
```

Your `index.html`'s script tag would look like this:

```html
<script type="module">
    import DuckityClient from "./js/duckity/duckity.js"
</script>
```
