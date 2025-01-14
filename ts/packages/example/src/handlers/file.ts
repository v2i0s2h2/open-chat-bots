import { Response } from "express";
import fs from "fs";
import mime from "mime-types";
import { placeholderResponse } from "./placeholder";
import { ExtendedRequest } from "../types";

async function processFile(filePath: string) {
  try {
    const buffer = fs.readFileSync(filePath);
    const uint8Array = new Uint8Array(buffer);
    const mimeType = mime.lookup(filePath) || "application/octet-stream";
    const fileSize = buffer.length;
    console.log(`File loaded successfully:`);
    console.log(`  MIME Type: ${mimeType}`);
    console.log(`  Size: ${(fileSize / 1024).toFixed(2)} KB`);

    return { uint8Array, mimeType, fileSize };
  } catch (err) {
    console.error("Error loading file:", err);
    throw err;
  }
}

export default async function file(req: ExtendedRequest, res: Response) {
  const client = req.botClient;
  const placeholder = "Uploading file ...";

  client.sendTextMessage(false, placeholder);

  res.status(200).json(placeholderResponse(client, placeholder, false));

  const filePath = "./dummy.pdf";
  const { uint8Array, fileSize, mimeType } = await processFile(filePath);

  client.sendFileMessage(
    true,
    filePath,
    uint8Array,
    mimeType,
    fileSize,
    "This is a test file"
  );
}
