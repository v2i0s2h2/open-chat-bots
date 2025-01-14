import { BotClient } from "@open-ic/openchat-botclient-ts";
import { success } from "./success";

export function placeholderResponse(
  client: BotClient,
  text: string,
  finalised: boolean
) {
  return success({
    id: client.messageId,
    content: {
      Text: {
        text,
      },
    },
    finalised,
  });
}
