import { CandidService } from "../utils/candidService";
import type { ExecuteBotCommandResponse, MessageContent } from "./candid/types";
import type { Chat } from "../storageIndex/candid/types";
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
export type BotCommandArgValue = BotCommandStringValue | BotCommandBooleanValue | BotCommandNumberValue | BotCommandUserValue;
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
export declare class BotClient extends CandidService {
    #private;
    private config;
    constructor(config: BotClientConfig);
    stringArg(name: string): string | undefined;
    booleanArg(name: string): boolean | undefined;
    numberArg(name: string): number | undefined;
    userArg(name: string): string | undefined;
    get commandArgs(): BotCommandArg[];
    get commandName(): string;
    get messageId(): string;
    get threadRootMessageId(): number | undefined | null;
    get chatId(): Chat;
    get initiator(): string;
    get botId(): string;
    createFileMessage(finalised: boolean, name: string, data: Uint8Array, mimeType: string, fileSize: number, caption?: string): Promise<Message>;
    sendFileMessage(finalised: boolean, name: string, data: Uint8Array, mimeType: string, fileSize: number, caption?: string): Promise<ExecuteBotCommandResponse>;
    createImageMessage(finalised: boolean, imageData: Uint8Array, mimeType: string, width: number, height: number, caption?: string): Promise<Message>;
    sendImageMessage(finalised: boolean, imageData: Uint8Array, mimeType: string, width: number, height: number, caption?: string): Promise<ExecuteBotCommandResponse>;
    createTextMessage(finalised: boolean, text: string): Promise<Message>;
    sendTextMessage(finalised: boolean, text: string): Promise<ExecuteBotCommandResponse>;
    sendMessage(message: Message): Promise<ExecuteBotCommandResponse>;
}
