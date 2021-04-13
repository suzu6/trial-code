extern crate num;
use num::Complex;


fn main(){
    let z = Complex { re: 1.0, im: 0.5 };
    let limit: u32 = 100;
    let result = complex_square_loop(z, limit);
    
    println!("{:?}", result);
    println!("Hello, world!");
}


#[allow(dead_code)]
fn complex_square_loop(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}
