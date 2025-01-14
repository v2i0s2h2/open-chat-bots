import { Response } from "express";
import { getSpotifyAccessToken, searchSpotifyArtists } from "./spotify";
import { placeholderResponse } from "./placeholder";
import { argumentsInvalid } from "@open-ic/openchat-botclient-ts";
import { ExtendedRequest } from "../types";

export default async function (req: ExtendedRequest, res: Response) {
  const client = req.botClient;
  const artist = client.stringArg("artist");
  if (artist === undefined) {
    res.status(400).send(argumentsInvalid());
  } else {
    const placeholder = `Searching Spotify for ${artist} ...`;

    client
      .sendTextMessage(false, placeholder)
      .catch((err) => console.error("sendTextMessage failed with: ", err));

    res.status(200).json(placeholderResponse(client, placeholder, false));

    const token = await getSpotifyAccessToken();
    const item = await searchSpotifyArtists(token, artist);
    const url = item.external_urls.spotify;

    client
      .sendTextMessage(true, url)
      .catch((err) => console.error("sendTextMessage failed with: ", err));
  }
}
