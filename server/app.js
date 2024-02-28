import { default as json, default as urlencoded } from "body-parser";
import express from "express";
import WebSocket from "ws";
import { start } from "./cue.js";

const USE_API = false;
const SUBS_DIR = "./subs";

let currentText = "";

const wss = new WebSocket.Server({ port: 8080 });
let connections = [];

wss.on("connection", function connection(ws) {
  console.log("connected");
  connections.push(ws);
});

const updateText = (text) => {
  if (text.startsWith("#") || (text.startsWith("[") && text.endsWith("]"))) {
    text = ""; // comment or blank
  }
  currentText = text;
  for (const ws of connections) {
    ws.send(text);
  }
};

const app = express();
const port = 3000;

app.get("/", (req, res) => {
  res.redirect("/index.html");
});

app.get("/current", (req, res) => {
  res.send(currentText);
});

app.use(express.static("public"));

if (USE_API) {
  app.use(urlencoded({ extended: false }));
  app.use(json());

  app.post("/", (req, res) => {
    if (req.body["text"] !== "" && !req.body["text"]) {
      res.status(400);
      res.send("No text!");
    } else {
      currentText = req.body["text"];
      res.send("Updated!");
      console.log(`Got text from ${req.ip}: ${currentText}`);
      updateText(currentText);
    }
  });
} else {
  start(updateText, SUBS_DIR);
}

app.listen(port, () => {
  console.log(`Subtitle server listening on port ${port}`);
});
