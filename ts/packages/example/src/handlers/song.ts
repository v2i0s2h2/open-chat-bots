import { Response } from "express";
import { getSpotifyAccessToken, searchSpotifySongs } from "./spotify";
import { placeholderResponse } from "./placeholder";
import { argumentsInvalid } from "@open-ic/openchat-botclient-ts";
import { ExtendedRequest } from "../types";

export default async function (req: ExtendedRequest, res: Response) {
  const client = req.botClient;
  const song = client.stringArg("song");
  if (song === undefined) {
    res.status(400).send(argumentsInvalid());
  } else {
    const placeholder = "Searching Spotify ...";

    client
      .sendTextMessage(false, placeholder)
      .catch((err: unknown) =>
        console.error("sendTextMessage failed with: ", err)
      );

    res.status(200).json(placeholderResponse(client, placeholder, false));

    const token = await getSpotifyAccessToken();
    const item = await searchSpotifySongs(token, song);
    const url = item.external_urls.spotify;

    client
      .sendTextMessage(true, url)
      .catch((err: unknown) =>
        console.error("sendTextMessage failed with: ", err)
      );
  }
}
