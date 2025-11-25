import { ChallengeRequest, waitForMessage } from "./worker.js";

export default class DuckityClient {
    domain: string;

    /**
     * Creates a new `DuckityClient` instance.
     *
     * @param {string} [domain = "quack.duckity.dev"] - The domain to connect to.
     */
    constructor(domain: string = "quack.duckity.dev") {
        this.domain = domain;
    }

    /**
     * Fetches, solves, and encodes a challenge from the server.
     *
     * @param {string} appId - The application ID.
     * @param {string} profileCode - The profile code.
     * @returns {Promise<string>} A promise that resolves to the encoded challenge solution.
     */
    public solve(appId: string, profileCode: string): Promise<string> {
        const workerUrl = new URL('./worker.js', import.meta.url);
        let worker = new Worker(workerUrl, { type: "module" });

        worker.postMessage({
            domain: this.domain,
            appId,
            profileCode,
        } as ChallengeRequest);

        return waitForMessage(worker);
    }
}
