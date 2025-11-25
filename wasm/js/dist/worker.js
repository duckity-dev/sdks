import init, * as wasm from "./wasm/duckity.js";
self.onmessage = async (event) => {
    await init();
    const { data } = event;
    const { domain, appId, profileCode } = data;
    let challenge = await wasm.DuckityClient.with_domain(domain).get_challenge(appId, profileCode);
    let solution = challenge.solve();
    let encoded = solution.encode();
    self.postMessage(encoded);
};
export function waitForMessage(worker) {
    return new Promise((resolve, reject) => {
        const successHandler = (event) => {
            worker.removeEventListener("message", successHandler);
            resolve(event.data);
        };
        const errorHandler = (ev) => {
            worker.removeEventListener("message", successHandler);
            worker.removeEventListener("error", errorHandler);
            reject(new Error(`An error occurred while solving a Duckity challenge: ${ev.error}`));
        };
        worker.addEventListener("message", successHandler);
        worker.addEventListener("error", errorHandler);
    });
}
