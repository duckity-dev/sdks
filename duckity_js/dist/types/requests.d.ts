/**
 * Makes a GET request to the specified URL and returns the parsed JSON response.
 *
 * If the request fails, it throws an error with a message extracted from the response if
 * available, or a generic message if not.
 *
 * @param url The URL to make the request to.
 * @param options Extra options to pass to the Fetch API, such as headers and body.
 * @returns The parsed JSON response from the server if the request is successful.
 */
export declare function get<T>(url: string, options?: RequestInit): Promise<T>;
/**
 * Makes a POST request to the specified URL and returns the parsed JSON response.
 *
 * If the request fails, it throws an error with a message extracted from the response if
 * available, or a generic message if not.
 *
 * @param url The URL to make the request to.
 * @param options Extra options to pass to the Fetch API, such as headers and body.
 * @returns The parsed JSON response from the server if the request is successful.
 */
export declare function post<T>(url: string, options?: Omit<RequestInit, "body" | "method"> & {
    body: any;
}): Promise<T>;
