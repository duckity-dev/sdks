import { useRef, useState } from "react";
import { post } from "./requests";
import wrapper from "./processing/wrapper";

interface SolveOptions {
  /**
   * Custom-Context Threat Correlation keys.
   */
  keys?: { [key: string]: string };
}

interface ChallengeResponse {
  /**
   * The challenge string that needs to be solved.
   */
  challenge: string;
}

/**
 * The result returned by the `useChallenge` hook, containing the solve function, current status,
 * solution, and boolean flags for loading, error, and idle states.
 */
interface UseChallengeResult {
  /**
   * Requests and solves a challenge for the specified application and protection profile.
   *
   * @param options Additional options to use when requesting a challenge.
   * @returns The already-solved and encoded challenge solution string.
   */
  solve: (options?: SolveOptions) => Promise<string>;

  /**
   * Discards the current challenge solution, if any.
   *
   * Note that if the challenge is currently being fetched or solved, calling this function will
   * not stop the ongoing process and the solution still will be set once that solve() call
   * completes.
   */
  invalidate: () => void;

  /**
   * Returns a promise that resolves when a challenge solution is available.
   *
   * If a solution is already available, the promise resolves immediately. If no solution is
   * available yet, the promise resolves once a solution is obtained through the solve() function.
   * 
   * If the solve() function encounters an error while fetching or solving the challenge, the
   * promise returned by this function will reject with the same error.
   *
   * @returns A promise that resolves to the current challenge solution string once it is available.
   */
  wait: () => Promise<string>;

  /**
   * The current status of the challenge-solving process, which can be one of "idle", "fetching",
   * "solving", "solved", or "error".
   */
  status: Status;

  /**
   * The latest solved challenge solution string, or null if no solution has been obtained yet.
   */
  solution: string | null;

  /**
   * Boolean flag indicating whether the challenge is currently being fetched or solved.
   */
  isLoading: boolean;

  /**
   * Boolean flag indicating whether an error occurred during the challenge-solving process.
   */
  isError: boolean;

  /**
   * Boolean flag indicating whether the challenge-solving process is currently idle (i.e., not
   * fetching, solving, or in an error state).
   */
  isIdle: boolean;
}

/**
 * The possible status values for the challenge-solving process.
 */
type Status = "idle" | "fetching" | "solving" | "solved" | "error";

interface Waiter {
  /**
   * Resolves the waiter with the provided solution string once it is available.
   * 
   * @param value The solution string to resolve with.
   */
  resolve: (value: string) => void;

  /**
   * Rejects the promise associated with the waiter with the provided reason if an error occurs
   * during fetching or solving.
   * 
   * @param reason The error reason to reject with.
   */
  reject: (reason?: any) => void;
}

/**
 * Hook to handle fetching and solving a challenge for a given application and protection profile.
 *
 * @param protectionProfileId The ID of the protection profile to use to get a challenge.
 * @returns An object containing the solve function, current status, solution, and boolean flags
 *   for loading, error, and idle states.
 */
export function useChallenge(
  protectionProfileId: string,
): UseChallengeResult {
  const [solution, setSolution] = useState<string | null>(null);
  const [status, setStatus] = useState<Status>("idle");
  const waiters = useRef<Array<Waiter>>([]);

  async function solve(options?: SolveOptions) {
    try {
      setStatus("fetching");

      let response: ChallengeResponse = await post(
        "https://quack.duckity.dev/v1/challenge",
        {
          body: JSON.stringify({
            id: protectionProfileId,
            keys: options?.keys,
          }),
        },
      );

      setStatus("solving");

      let solution = await wrapper.solve(response.challenge);

      for (let waiter of waiters.current) {
        waiter.resolve(solution);
      }
      waiters.current = [];

      setStatus("solved");

      setSolution(solution);

      return solution;
    } catch (error) {
      setStatus("error");

      for (let waiter of waiters.current) {
        waiter.reject(error);
      }
      waiters.current = [];

      throw error;
    }
  }

  function invalidate() {
    setSolution(null);
  }

  function wait(): Promise<string> {
    if (solution) {
      return Promise.resolve(solution);
    } else {
      return new Promise((resolve, reject) => {
        waiters.current.push({
          resolve,
          reject,
        });
      });
    }
  }

  return {
    solve,
    invalidate,
    wait,
    status,
    solution,
    isLoading: status === "fetching" || status === "solving",
    isError: status === "error",
    isIdle: status === "idle",
  };
}
