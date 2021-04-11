fn main() {
    proconio::input! {
        r: f64,
        x: f64,
        y: f64,
    }

    let distance:f64 = x*x + y*y;
    let mut upper_limit: f64 = distance.sqrt();
    let mut count = 1;
    let ep = 0.000000001;
    // println!("{}", upperLimit);
    while upper_limit > r + ep {
        upper_limit -= r;
        count += 1;
    }
    if count == 1 && upper_limit != r {
        println!("{}", count+1);
    }else{
        println!("{}", count);
    }

}
