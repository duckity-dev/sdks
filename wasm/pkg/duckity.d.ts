/* tslint:disable */
/* eslint-disable */
/**
 * A Duckity challenge.
 *
 * Use `Challenge.solve()` to solve the challenge and get a `Solution`.
 */
export class Challenge {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Solve the Duckity challenge.
   *
   * Note that this operation is computationally-intensive. Make sure to run it from a worker
   * not to block the main thread.
   *
   * Returns:
   * * `Solution` - The solution to the challenge.
   */
  solve(): Solution;
}
/**
 * A client for interacting with the Duckity API.
 *
 * To create a new client, use `DuckityClient.new()`. If you're using a self-hosted duckling,
 * use `DuckityClient.with_domain()` to point to your custom domain.
 *
 * To get a challenge, use `DuckityClient.get_challenge()`. To solve it, use
 * `Challenge.solve()`. Use `Solution.encode()` to get the encoded solution string.
 */
export class DuckityClient {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create a new Duckity client pointing to a custom domain.
   *
   * Use this if you're self-hosting Duckity or using a different environment.
   *
   * Arguments:
   * * `domain` - The domain to point the client to.
   *
   * Returns:
   * `DuckityClient` - A new Duckity client.
   */
  static with_domain(domain: string): DuckityClient;
  /**
   * Get a challenge for the given application ID and profile code.
   *
   * Arguments:
   * * `app_id` - The application ID to get the challenge for.
   * * `profile_code` - The profile code to use for the challenge.
   *
   * Returns:
   * * `Challenge` - The solution token, if successful.
   * * `String` - An error string if the request failed.
   */
  get_challenge(app_id: string, profile_code: string): Promise<Challenge>;
  /**
   * Create a new Duckity client.
   *
   * Use `DuckityClient.with_domain()` instead if you want to point to a custom domain.
   *
   * Returns:
   * `DuckityClient` - A new Duckity client.
   */
  constructor();
}
/**
 * The solution to a Duckity challenge.
 */
export class Solution {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Encode the solution as a base64 URL-safe string.
   *
   * Returns:
   * * `String` - The encoded solution.
   */
  encode(): string;
  /**
   * Get the raw size of the solution in bytes.
   *
   * Returns:
   * * `Number` - The size of the solution in bytes.
   */
  raw_size(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_challenge_free: (a: number, b: number) => void;
  readonly __wbg_duckityclient_free: (a: number, b: number) => void;
  readonly __wbg_solution_free: (a: number, b: number) => void;
  readonly challenge_solve: (a: number) => number;
  readonly duckityclient_get_challenge: (a: number, b: number, c: number, d: number, e: number) => any;
  readonly duckityclient_new: () => number;
  readonly duckityclient_with_domain: (a: number, b: number) => number;
  readonly solution_encode: (a: number) => [number, number];
  readonly solution_raw_size: (a: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__hd181342ed75b9bd0: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h52651d6625bdd2b3: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h2e0445ef21469310: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
