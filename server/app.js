const express = require("express");
const bodyParser = require("body-parser");
const WebSocket = require("ws");
const cue = require("./cue");

const USE_API = false;

let currentText = "";

const wss = new WebSocket.Server({ port: 8080 });
let connections = [];

wss.on("connection", function connection(ws) {
  console.log("connected");
  connections.push(ws);
});

const updateText = (text) => {
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
  app.use(bodyParser.urlencoded({ extended: false }));
  app.use(bodyParser.json());

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
}

app.listen(port, () => {
  console.log(`Subtitle server listening on port ${port}`);
});

if (!USE_API) {
  cue.start(updateText);
}
