const axios = require('axios');
const CancelToken = axios.CancelToken;
const source = CancelToken.source();
const source2 = CancelToken.source();

const start = Date.now();
console.log("start: ", start)

// レスポンスがそんなかからないリクエスト
axios.get('http://localhost:3000/', {
  cancelToken: source.token
}).then((res) => {
    // 正常動作
    console.log(Date.now() - start, res.data);
}).catch((thrown) => {
  if (axios.isCancel(thrown)) {
    console.log(Date.now() - start, 'Request canceled: ', thrown.message);
  } else {
    // handle error
    console.log(Date.now() - start, 'Some error: ', thrown.message);
  }
})

// レスポンスに３秒かかるリクエスト
axios.get('http://localhost:3000/delay', {
  cancelToken: source.token
}).then((res) => {
    // 正常動作
    console.log(Date.now() - start, res.data);
}).catch((thrown) => {
  if (axios.isCancel(thrown)) {
    console.log(Date.now() - start, 'Request canceled: ', thrown.message);
  } else {
    // handle error
    console.log(Date.now() - start, 'Some error: ', thrown.message);
  }
})


// レスポンスに３秒かかるリクエスト（キャンセルしない）
axios.get('http://localhost:3000/delay', {
  cancelToken: source2.token
})
.then((res) => {
    // 正常動作
    console.log(Date.now() - start, res.data);
}).catch((thrown) => {
  if (axios.isCancel(thrown)) {
    console.log(Date.now() - start, 'Request canceled: ', thrown.message);
  } else {
    // handle error
    console.log(Date.now() - start, 'Some error: ', thrown.message);
  }
})

// 1秒後にキャンセルリクエスト
setTimeout(() => {
    // cancel the request (the message parameter is optional)
    source.cancel('Operation canceled by the user.');
}, 1000);