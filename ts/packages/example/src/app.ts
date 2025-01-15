/**
 * Create a simple express js server. This should have two endpoints:
 * /execute_command - to receive commands as POST requests from the OpenChat front end
 * / - to receive any GET request from the OpenChat front end (or elsewhere) and return the bot's schema definition
 */
import express from "express";
import cors from "cors";
import executeCommand from "./handlers/executeCommand";
import schema from "./handlers/schema";
import createBotClient from "./middleware/botclient";
import { rateLimit } from "express-rate-limit";

const limiter = rateLimit({
  windowMs: 1 * 60 * 1000,
  limit: 3, // 3 per minute
  standardHeaders: "draft-8",
  legacyHeaders: false,
  statusCode: 429,
});

const app = express();

/**
 * Here, some of the various arguments needed to create an instance of the BotClient are retrieved from environment variables.
 * See the readme for more information.
 */
const OC_PUBLIC = process.env.OC_PUBLIC!;
const IDENTITY_PRIVATE = process.env.IDENTITY_PRIVATE!;
const IC_HOST = process.env.IC_HOST!;
const STORAGE_INDEX_CANISTER = process.env.STORAGE_INDEX_CANISTER!;

app.use(cors());
app.use(limiter);
app.use(express.text());
app.post(
  "/execute_command",
  createBotClient(OC_PUBLIC, IDENTITY_PRIVATE, IC_HOST, STORAGE_INDEX_CANISTER), // insert the middleware that will create the OpenChat BotClient
  executeCommand
);
app.get("/", schema);

export default app;
