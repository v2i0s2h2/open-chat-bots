/**
 * This is an express middleware to create an instance of the OpenChat BotClient
 * which can be used for the duration of a single request to interact with the OpenChat backend.
 * See the readme for more explanation.
 */
import { Request, Response, NextFunction } from "express";
import { BotClient, BadRequestError } from "@open-ic/openchat-botclient-ts";
import { ExtendedRequest } from "../types";

export default function createBotClient(
  ocPublic: string,
  privateKey: string,
  icHost: string,
  storageIndexCanister: string
) {
  return (req: Request, res: Response, next: NextFunction): void => {
    try {
      (req as ExtendedRequest).botClient = new BotClient({
        openStorageCanisterId: storageIndexCanister,
        icHost,
        identityPrivateKey: privateKey,
        openchatPublicKey: ocPublic,
        encodedJwt: req.body,
      });
      console.log("Bot client created");
      next();
    } catch (err: any) {
      console.log("Error creating bot client: ", err);
      if (err instanceof BadRequestError) {
        res.status(400).send(err.message);
      } else {
        res.status(500).send(err.message);
      }
    }
  };
}
