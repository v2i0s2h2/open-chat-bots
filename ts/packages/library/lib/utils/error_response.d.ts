export type BadRequest = "AccessTokenNotFound" | "AccessTokenInvalid" | "AccessTokenExpired" | "CommandNotFound" | "ArgsInvalid";
export declare function argumentsInvalid(): BadRequest;
export declare function accessTokenNotFound(): BadRequest;
export declare function accessTokenInvalid(): BadRequest;
export declare function accessTokenExpired(): BadRequest;
export declare function commandNotFound(): BadRequest;
export declare class BadRequestError extends Error {
    constructor(message: BadRequest);
}
export declare class InternalServerError extends Error {
}
