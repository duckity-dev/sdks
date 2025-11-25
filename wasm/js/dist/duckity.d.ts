export default class DuckityClient {
    domain: string;
    /**
     * Creates a new `DuckityClient` instance.
     *
     * @param {string} [domain = "quack.duckity.dev"] - The domain to connect to.
     */
    constructor(domain?: string);
    /**
     * Fetches, solves, and encodes a challenge from the server.
     *
     * @param {string} appId - The application ID.
     * @param {string} profileCode - The profile code.
     * @returns {Promise<string>} A promise that resolves to the encoded challenge solution.
     */
    solve(appId: string, profileCode: string): Promise<string>;
}
