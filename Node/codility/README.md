# Codfility test

テストしながら開発できるようにする

## nodeの切替

```bash
> nvm -v 
Running version 1.1.7.

# nvmで8.9.6は取り扱ってないので8.9をインストールする
> nvm install 8.9

> nvm list

  * 14.17.5 (Currently using 64-bit executable)
    8.9.0

# node切り替え
> nvm use 8.9.0

> node -v
v8.9.0
> npm -v       
5.5.1
```


作業が終わったら
```bashs
> nvm use 14.17.5
```

## テスト
node 8.9では古くてjestが動かなかった。めんどいのでv14に戻す。
```bash
> npm install --save-dev jest
> npm test   

> my-codility@1.0.0 test C:\git\my\trial-code\Node\codility
> jest

C:\git\my\trial-code\Node\codility\node_modules\jest\node_modules\jest-cli\build\cli\index.js:219
    } catch {
            ^

SyntaxError: Unexpected token {
    at createScript (vm.js:80:10)
    at Object.runInThisContext (vm.js:139:10)
    at Module._compile (module.js:599:28)
    at Object.Module._extensions..js (module.js:646:10)
    at Module.load (module.js:554:32)
    at tryModuleLoad (module.js:497:12)
    at Function.Module._load (module.js:489:3)
    at Module.require (module.js:579:17)
    at require (internal/module.js:11:18)
    at Object.<anonymous> (C:\git\my\trial-code\Node\codility\node_modules\jest\node_modules\jest-cli\bin\jest.js:16:3)
npm ERR! Test failed.  See above for more details.
```