mod math;

fn main() {
    let n = 10;
    let result = math::sum_one_to_n(n);
    println!("{}", result);

    let mut v :Vec<i32> = vec![3, 4, 5];
    v.push(6);
}
