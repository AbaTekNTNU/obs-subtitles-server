const express = require('express');
const bodyParser = require('body-parser');
const WebSocket = require('ws');

const wss = new WebSocket.Server({ port: 8080 });

let connections = [];

wss.on('connection', function connection(ws) {
    console.log('connected');
    connections.push(ws);
});

updateText = (text) => {
    for (const ws of connections) {
        ws.send(text)
    }
}

const app = express();
const port = 3000;

let currentText = '';

app.use(bodyParser.urlencoded({extended: false}));
app.use(bodyParser.json())

app.get('/', (req, res) => {
    res.send(currentText);
});

app.use(express.static('public'))

app.post('/', (req, res) => {
    if (req.body['text'] !== '' && !req.body['text']) {
        res.status(400);
        res.send('No text!');
    } else {
        currentText = req.body['text'];
        res.send('Updated!');
        updateText(currentText);
    }
});

app.listen(port, () => {
    console.log(`Example app listening at http://localhost:${port}`);
});
