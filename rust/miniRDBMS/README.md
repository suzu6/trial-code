# miniRDBMS

これはWEB+DB vol122の「作って学ぶRDBMSのしくみ」のサンプルを動かしている奴。

公開されているコード
https://github.com/KOBA789/relly

は本人が動画を出しているので、こちらを見てもらうと内容についてわかりやすいです。
これ見てから本を買いました。
https://www.youtube.com/watch?v=eWDkIz9BN0A

## 実装してみて

書籍のサンプルを写経して実行はできない。
GuHubのコードとCargo.tomlのクレートの依存関係を書いておかないとエラーになる。

ファイルをコピペしてきて、処理ごとにコメントを書くのが良いかも。

ひとファイルずつ書いてテストするには`main.rs`に`mod disk;`と書いてから`cargo test`を実行する必要がある。

```rust:main.rs
mod disk;
```
上述がないとテスト出来ない。（`cargo test disk`もだめ）
```bash
> cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running target\debug\deps\mini_rdbms-84b4747aa01fcfe2.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


とりあえず、