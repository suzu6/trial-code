fn main() {
    println!("Hello, world!");
    // 変数
    // let x = 5;
    // error[E0384]: cannot assign twice to immutable variable `x`
    // 可変変数 Mutability
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    // 定数 contants
    const MAX_POINTS: u32 = 100_000;
    println!("The constant of MAX_POINTS is: {}", MAX_POINTS);

    // シャドーイング
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    // 同じな名前で宣言できるため気を付ける

    let guess: u32 = "42".parse().expect("Not a number!"); // 数字ではありません！
    println!("guess is: {}", guess);

    // let a = [1, 2, 3, 4, 5];
    // let index = 10;

    // let element = a[index];
    // index out of bounds: the length is 5 but the index is 10

    // println!("The value of element is: {}", element);   // 要素の値は{}です

    another_function(5);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    // 発射！
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // 値は{}です
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for number in (1..4).rev() { // rev(): reverce
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
fn another_function(x: i32) {
    println!("Another function {}", {
        let y = 3;
        x + y + 1 // ;がないとreturnになる
    });
}
