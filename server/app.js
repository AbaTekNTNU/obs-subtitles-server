const express = require('express');
const bodyParser = require('body-parser');

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
    }
});

app.listen(port, () => {
    console.log(`Example app listening at http://localhost:${port}`);
});
