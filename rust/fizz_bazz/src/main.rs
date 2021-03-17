fn main() {
    const N:i32 = 20;

    fizz_bazz(N);
}

/**
 * オーソドックスなFizzBazz
 */
fn fizz_bazz(number: i32){

    for n in 1..number {
        if n % 15 == 0 {
            println!("FizzBazz");
        }else if n % 3 == 0 {
            println!("Fizz");
        }else if n % 5 == 0 {
            println!("Bazz");
        }else {
            println!("{}", n);
        }
    }
}
