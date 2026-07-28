import * as Comlink from "comlink";
import { process } from "@duckity/wasm";

/**
 * Expose the `process` function from the WASM module to the main thread via Comlink.
 *
 * This allows the main thread to call the `process` function in the worker thread, which will
 * handle the processing of the challenges without blocking the main thread.
 */
const api = {
  async process(challenge: string) {
    return process(challenge);
  },
};

Comlink.expose(api);
