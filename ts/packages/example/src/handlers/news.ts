import { Response } from "express";
import { ExtendedRequest } from "../types";
import { success } from "./success";
import * as cheerio from "cheerio";

type NewsSummary = {
  title: string;
  link: string;
};

async function scrapeBBCNews(): Promise<NewsSummary[]> {
  const url = "https://www.bbc.com/news";

  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error("Unable to load the bbc website");
    }

    const data = await response.text();
    const $ = cheerio.load(data);
    const newsSummaries: { title: string; link: string }[] = [];

    $("h3").each((_, element) => {
      const title = $(element).text();
      const link = $(element).find("a").attr("href");
      if (link && !link.startsWith("http")) {
        newsSummaries.push({
          title,
          link: `https://www.bbc.com${link}`,
        });
      } else if (link) {
        newsSummaries.push({ title, link });
      }
    });

    return newsSummaries;
  } catch (error) {
    console.error("Failed to retrieve news:", error);
    return [];
  }
}

function toMarkdown(news: NewsSummary[]): string {
  return news.map(({ title, link }) => `[${title}](${link})`).join("\n");
}

export default async function news(req: ExtendedRequest, res: Response) {
  const client = req.botClient;
  const placeholder = await client.createTextMessage(
    false,
    "Searching for the news ..."
  );

  client
    .sendMessage(placeholder)
    .catch((err: unknown) => console.error("sendMessage failed with: ", err));

  res.status(200).json(success(placeholder));

  const news = await scrapeBBCNews();

  client
    .sendTextMessage(true, toMarkdown(news.slice(0, 10)))
    .catch((err: unknown) =>
      console.error("sendTextMessage failed with: ", err)
    );
}
