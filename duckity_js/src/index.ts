import wrapper from "./processing/wrapper";
import { post } from "./requests";

/**
 * Options for getting a Duckity challenge from the API.
 */
export interface GetDuckityChallengeOptions {
  /**
   * The custom-context threat correlation keys to be sent with the request.
   */
  keys?: { [key: string]: string };

  /**
   * The base URL to the API endpoint. Defaults to `https://quack.duckity.com` if not provided.
   *
   * Only update this when self-hosting a Duckling.
   */
  api?: string;
}

/**
 * The response from the Duckling API when requesting a challenge.
 */
interface ChallengeResponse {
  /**
   * The encoded challenge string returned by the Duckling API.
   */
  challenge: string;
}

/**
 * Fetches, solves, and returns the solution to a Duckity challenge for the given protection
 * profile ID.
 *
 * @param protectionProfileId The ID of the protection profile to get the challenge for.
 * @param options Optional parameters for the challenge issuance.
 * @returns The solution to the challenge issued by Duckity.
 */
export async function solve(
  protectionProfileId: string,
  options?: GetDuckityChallengeOptions,
): Promise<string> {
  if (options?.api && options.api.endsWith("/")) {
    // Remove the trailing slash from the API URL if provided
    options.api = options.api.slice(0, -1);
  }

  let response: ChallengeResponse = await post(
    `${options?.api || "https://quack.duckity.com"}/v1/challenge`,
    {
      body: {
        id: protectionProfileId,
        keys: options?.keys || {},
      },
    },
  );

  let solution = await wrapper.solve(response.challenge);

  return solution;
}

export default { solve }
