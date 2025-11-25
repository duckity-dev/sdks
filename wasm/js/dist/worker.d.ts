export interface ChallengeRequest {
    domain: string;
    appId: string;
    profileCode: string;
}
export declare function waitForMessage(worker: Worker): Promise<string>;
