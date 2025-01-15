export type BadRequest = "AccessTokenNotFound" | "AccessTokenInvalid" | "AccessTokenExpired" | "CommandNotFound" | "TooManyRequests" | "ArgsInvalid";
export declare function argumentsInvalid(): BadRequest;
export declare function accessTokenNotFound(): BadRequest;
export declare function accessTokenInvalid(): BadRequest;
export declare function accessTokenExpired(): BadRequest;
export declare function commandNotFound(): BadRequest;
export declare function tooManyRequests(): BadRequest;
export declare class BadRequestError extends Error {
    constructor(message: BadRequest);
}
