const express = require('express')
var bodyParser = require('body-parser');
const app = express()
const port = 3000


app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());

// http://localhost:3000/ でresponce.jsonの中身を返す。
app.get("/", (req, res, next) => {
    // アクセスログ
    console.log(req.method, req.url)
    console.log(req.headers)
    // jsonを返す
    // res.json(require('./responce.json'))
    res.send("OK")
});

// ポート3000でサーバ起動

app.post("/user", (req, res, next) => {
    // アクセスログ
    console.log(req.method, req.url, req.ip)

    // headerを表示
    console.log(req.headers)

    // bodyを表示
    console.log(req.body)

    // 登録処理など

    res.send('OK')
});

app.listen(port, () => console.log(`click http://localhost:${port}/ !`))