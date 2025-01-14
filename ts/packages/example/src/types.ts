import { BotClient } from "@open-ic/openchat-botclient-ts";
import { Request } from "express";

export interface ExtendedRequest extends Request {
  botClient: BotClient;
}
