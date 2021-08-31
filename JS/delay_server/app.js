const express = require('express')
const app = express()
const port = 3000

app.get('/', (req, res) => {
  res.send('Hello World!')
})

// リクエスト返答まで時間がかかるエンドポイント
app.get('/delay', (req, res) => {
  setTimeout(()=>{
    res.send('3 seconds delay');
  }, 3000);
})

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})