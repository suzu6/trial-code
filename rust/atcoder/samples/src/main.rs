use num::integer;

fn main() {
    println!("Hello, world!");


    let a = 12;
    let b = 18;
    
    println!("{}", integer::sqrt(a));


    println!("{}", integer::gcd(a, b));
    println!("{}", integer::lcm(a, b));
}
