import * as Comlink from "comlink";
/**
 * Wrap the worker with Comlink to allow for easy communication between the main thread and the
 * worker thread. The worker exposes a `solve` function that takes a challenge string and returns a
 * promise that resolves to the solution string.
 */
declare const _default: Comlink.Remote<{
    solve(challenge: string): Promise<string>;
}>;
export default _default;
