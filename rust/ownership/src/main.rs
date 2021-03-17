fn main() {
    let s1 = String::from("hello");

    // error: aborting due to previous error; 1 warning emitted
    // let s2 = s1;
    let s2 = s1.clone();

    println!("{}, {}, world!", s1, s2);

    // サイズが決まってるものはそのまま渡せる
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    

    
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);

    
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}

fn calculate_length2(s: &String) -> usize { // sはStringへの参照
    s.len()
}
