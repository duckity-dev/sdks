import { waitForMessage } from "./worker.js";
export default class DuckityClient {
    domain;
    /**
     * Creates a new `DuckityClient` instance.
     *
     * @param {string} [domain = "quack.duckity.dev"] - The domain to connect to.
     */
    constructor(domain = "quack.duckity.dev") {
        this.domain = domain;
    }
    /**
     * Fetches, solves, and encodes a challenge from the server.
     *
     * @param {string} appId - The application ID.
     * @param {string} profileCode - The profile code.
     * @returns {Promise<string>} A promise that resolves to the encoded challenge solution.
     */
    solve(appId, profileCode) {
        const workerUrl = new URL('./worker.js', import.meta.url);
        let worker = new Worker(workerUrl, { type: "module" });
        worker.postMessage({
            domain: this.domain,
            appId,
            profileCode,
        });
        return waitForMessage(worker);
    }
}
