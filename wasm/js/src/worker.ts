import init, * as wasm from "./wasm/duckity.js";

export interface ChallengeRequest {
    domain: string;
    appId: string;
    profileCode: string;
}

self.onmessage = async (event: MessageEvent<ChallengeRequest>) => {
    await init();

    const { data } = event;

    const { domain, appId, profileCode } = data;

    let challenge = await wasm.DuckityClient.with_domain(domain).get_challenge(
        appId,
        profileCode
    );

    let solution = challenge.solve();
    let encoded = solution.encode();

    self.postMessage(encoded);
};

export function waitForMessage(worker: Worker): Promise<string> {
    return new Promise((resolve, reject) => {
        const successHandler = (event: MessageEvent<string>) => {
            worker.removeEventListener("message", successHandler);
            resolve(event.data);
        };
        const errorHandler = (ev: ErrorEvent) => {
            worker.removeEventListener("message", successHandler);
            worker.removeEventListener("error", errorHandler);

            reject(
                new Error(
                    `An error occurred while solving a Duckity challenge: ${ev.error}`
                )
            );
        };

        worker.addEventListener("message", successHandler);
        worker.addEventListener("error", errorHandler);
    });
}
