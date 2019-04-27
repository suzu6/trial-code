/**
 * TechCommitの課題
 * 
 * 素数の判定
 * Nodeのコマンドライン引数を知りたかった。[Node.jsでコマンドライン引数を取得する - Qiita](https://qiita.com/furusin_oriver/items/f030d1eaa9e7b54233c3)
 */
const input = process.argv[2];

// 引数は全てstring
// console.log(typeof(input));
let num = parseInt(input);

if(!Number.isInteger(num) || num < 2){
    console.log("2以上の整数を渡してください");
    return;
}

console.log(num);

// 試し割り法 （2以上の整数のみ）
function trial_division(n){
    // 偶数
    if(n % 2 === 0) return 2;

    let max = Math.ceil(Math.log(n));
    // 奇数のみ
    for(let i = 3; i <= max; i += 2 ){
        if(n % i === 0) return i;
    }
    return 0;
}

let result = trial_division(num);

if(result === 0){
    console.log(num, "は素数です");
}else {
    console.log(num, "は", result,"で割り切れます")
}