/**
 * Options for getting a Duckity challenge from the API.
 */
interface GetDuckityChallengeOptions {
    /**
     * The custom-context threat correlation keys to be sent with the request.
     */
    keys?: {
        [key: string]: string;
    };
    /**
     * The base URL to the API endpoint. Defaults to `https://quack.duckity.com` if not provided.
     *
     * Update this when self-hosting a Duckling. No trailing slash must be included in the URL.
     */
    api?: string;
}
/**
 * Fetches, solves, and returns the solution to a Duckity challenge for the given protection
 * profile ID.
 *
 * @param protectionProfileId The ID of the protection profile to get the challenge for.
 * @param options Optional parameters for the challenge issuance.
 * @returns The solution to the challenge issued by Duckity.
 */
declare function solve(protectionProfileId: string, options?: GetDuckityChallengeOptions): Promise<string>;
declare const _default: {
    solve: typeof solve;
};

export { _default as default, solve };
export type { GetDuckityChallengeOptions };
