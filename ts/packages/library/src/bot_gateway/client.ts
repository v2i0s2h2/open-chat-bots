import { HttpAgent } from "@dfinity/agent";
import { CandidService } from "../utils/candidService";
import { type BotService, idlFactory } from "./candid/idl";
import type { ExecuteBotCommandResponse, MessageContent } from "./candid/types";
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";
import { DataClient } from "../data/data.client";
import { Principal } from "@dfinity/principal";
import type { Chat } from "../storageIndex/candid/types";
import jwt from "jsonwebtoken";
import { BadRequestError } from "../utils/badrequest";

export type Message = {
    id: string;
    content: MessageContent;
    finalised: boolean;
};

export type DecodedJwt = {
    exp: number;
    initiator: string;
    bot: string;
    chat: Chat;
    thread_root_message_index: number | null;
    message_id: string;
    command: {
        name: string;
        args: BotCommandArg[];
    };
    bot_api_gateway: string;
};

export type BotCommandArg = {
    name: string;
    value: BotCommandArgValue;
};

export type BotCommandArgValue =
    | BotCommandStringValue
    | BotCommandBooleanValue
    | BotCommandNumberValue
    | BotCommandUserValue;

export type BotCommandStringValue = {
    String: string;
};

export type BotCommandBooleanValue = {
    Boolean: boolean;
};

export type BotCommandNumberValue = {
    Number: number;
};

export type BotCommandUserValue = {
    User: Uint8Array;
};

export type BotClientConfig = {
    openStorageCanisterId: string;
    icHost: string;
    identityPrivateKey: string;
    openchatPublicKey: string;
    encodedJwt: string;
};

export class BotClient extends CandidService {
    #botService: BotService;
    #agent: HttpAgent;
    #identity: Secp256k1KeyIdentity;
    #decodedJwt: DecodedJwt;
    #encodedJwt: string;

    constructor(private config: BotClientConfig) {
        super();
        this.#validateConfig(config);
        this.#encodedJwt = config.encodedJwt;
        this.#decodedJwt = this.#decodeJwt(config.encodedJwt);
        this.#identity = this.#createIdentity(config.identityPrivateKey);
        console.log("Principal: ", this.#identity.getPrincipal().toText());
        this.#agent = new HttpAgent({
            identity: this.#identity,
            host: config.icHost,
            retryTimes: 5,
        });

        this.#botService = this.createServiceClient<BotService>(
            idlFactory,
            this.#decodedJwt.bot_api_gateway,
            config.icHost,
            this.#agent,
        );
    }

    #validateConfig(config: BotClientConfig) {
        if (config.encodedJwt === undefined) {
            throw new BadRequestError("AccessTokenNotFound");
        } else if (config.icHost === undefined) {
            throw new Error("IC Host not provided");
        } else if (config.identityPrivateKey === undefined) {
            throw new Error("Identity private key not provided");
        } else if (config.openStorageCanisterId === undefined) {
            throw new Error("OpenStorage index canister not provided");
        } else if (config.openchatPublicKey === undefined) {
            throw new Error("OpenChat public key not provided");
        }
    }

    #principalBytesToString(bytes: Uint8Array): string {
        return Principal.fromUint8Array(bytes).toString();
    }

    #namedArg(name: string): BotCommandArg | undefined {
        return this.#decodedJwt.command.args.find((a) => a.name === name);
    }

    public stringArg(name: string): string | undefined {
        const arg = this.#namedArg(name);
        return arg !== undefined && "String" in arg.value ? arg.value.String : undefined;
    }

    public booleanArg(name: string): boolean | undefined {
        const arg = this.#namedArg(name);
        return arg !== undefined && "Boolean" in arg.value ? arg.value.Boolean : undefined;
    }

    public numberArg(name: string): number | undefined {
        const arg = this.#namedArg(name);
        return arg !== undefined && "Number" in arg.value ? arg.value.Number : undefined;
    }

    public userArg(name: string): string | undefined {
        const arg = this.#namedArg(name);
        return arg !== undefined && "User" in arg.value
            ? this.#principalBytesToString(arg.value.User)
            : undefined;
    }

    public get commandArgs(): BotCommandArg[] {
        return this.#decodedJwt.command.args;
    }

    public get commandName(): string {
        return this.#decodedJwt.command.name;
    }

    public get messageId(): string {
        return this.#decodedJwt.message_id;
    }

    public get threadRootMessageId(): number | undefined | null {
        return this.#decodedJwt.thread_root_message_index;
    }

    public get chatId(): Chat {
        return this.#decodedJwt.chat;
    }

    public get initiator(): string {
        return this.#decodedJwt.initiator;
    }

    public get botId(): string {
        return this.#decodedJwt.bot;
    }

    #decodeJwt(token: string): DecodedJwt {
        const publicKey = this.config.openchatPublicKey.replace(/\\n/g, "\n");
        try {
            const decoded = jwt.verify(token, publicKey, { algorithms: ["ES256"] });
            if (typeof decoded !== "string") {
                return decoded as DecodedJwt;
            } else {
                console.error(`Unable to decode jwt`, token);
                throw new BadRequestError("AccessTokenInvalid");
            }
        } catch (err) {
            console.error(`Unable to decode jwt`, err, token);
            throw new BadRequestError("AccessTokenInvalid");
        }
    }

    #createIdentity(privateKey: string) {
        const privateKeyPem = privateKey.replace(/\\n/g, "\n");
        try {
            return Secp256k1KeyIdentity.fromPem(privateKeyPem);
        } catch (err) {
            console.error("Unable to create identity from private key", err);
            throw err;
        }
    }

    #extractCanisterFromChat() {
        if ("Group" in this.#decodedJwt.chat) {
            return this.#decodedJwt.chat.Group.toString();
        } else if ("Channel" in this.#decodedJwt.chat) {
            return this.#decodedJwt.chat.Channel[0].toString();
        }
        return "";
    }

    createFileMessage(
        finalised: boolean,
        name: string,
        data: Uint8Array,
        mimeType: string,
        fileSize: number,
        caption?: string,
    ): Promise<Message> {
        const dataClient = new DataClient(this.#agent, this.config);
        const canisterId = this.#extractCanisterFromChat();
        const uploadContentPromise = dataClient.uploadData([canisterId], mimeType, data);

        return uploadContentPromise.then((blobRef) => {
            return {
                id: this.messageId,
                content: {
                    File: {
                        name,
                        file_size: fileSize,
                        mime_type: mimeType,
                        blob_reference: [
                            {
                                blob_id: blobRef.blobId,
                                canister_id: Principal.fromText(blobRef.canisterId),
                            },
                        ],
                        caption: caption ? [caption] : [],
                    },
                },
                finalised,
            };
        });
    }

    sendFileMessage(
        finalised: boolean,
        name: string,
        data: Uint8Array,
        mimeType: string,
        fileSize: number,
        caption?: string,
    ): Promise<ExecuteBotCommandResponse> {
        return this.createFileMessage(finalised, name, data, mimeType, fileSize, caption).then(
            (msg) => this.sendMessage(msg),
        );
    }

    createImageMessage(
        finalised: boolean,
        imageData: Uint8Array,
        mimeType: string,
        width: number,
        height: number,
        caption?: string,
    ): Promise<Message> {
        const dataClient = new DataClient(this.#agent, this.config);
        const canisterId = this.#extractCanisterFromChat();
        console.log("Upload canister: ", canisterId);
        const uploadContentPromise = dataClient.uploadData([canisterId], mimeType, imageData);

        return uploadContentPromise.then((blobRef) => {
            return {
                id: this.messageId,
                content: {
                    Image: {
                        height,
                        mime_type: mimeType,
                        blob_reference: [
                            {
                                blob_id: blobRef.blobId,
                                canister_id: Principal.fromText(blobRef.canisterId),
                            },
                        ],
                        thumbnail_data: "",
                        caption: caption ? [caption] : [],
                        width,
                    },
                },
                finalised,
            };
        });
    }

    sendImageMessage(
        finalised: boolean,
        imageData: Uint8Array,
        mimeType: string,
        width: number,
        height: number,
        caption?: string,
    ): Promise<ExecuteBotCommandResponse> {
        return this.createImageMessage(finalised, imageData, mimeType, width, height, caption).then(
            (msg) => this.sendMessage(msg),
        );
    }

    createTextMessage(finalised: boolean, text: string): Promise<Message> {
        return Promise.resolve({
            id: this.messageId,
            content: {
                Text: { text },
            },
            finalised,
        });
    }

    sendTextMessage(finalised: boolean, text: string): Promise<ExecuteBotCommandResponse> {
        return this.createTextMessage(finalised, text).then((msg) => this.sendMessage(msg));
    }

    sendMessage(message: Message): Promise<ExecuteBotCommandResponse> {
        return this.#executeAction(message);
    }

    #executeAction(message: Message): Promise<ExecuteBotCommandResponse> {
        return this.handleResponse(
            this.#botService.execute_bot_action({
                jwt: this.#encodedJwt,
                action: {
                    SendMessage: message,
                },
            }),
            (res) => {
                if (!("Ok" in res)) {
                    console.error("Call to execute_bot_action failed with: ", JSON.stringify(res));
                }
                return res;
            },
        ).catch((err) => {
            console.error("Call to execute_bot_action failed with: ", JSON.stringify(err));
            throw err;
        });
    }
}
