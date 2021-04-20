extern crate num;
use num::integer;
use num::Float;
use num::Complex;

fn main() {
    println!("Hello, world!");


    // div_floor
    assert_eq!(integer::div_floor(7, 3), 2);
    assert_eq!(integer::div_ceil(7, 3), 3);
    assert_eq!(integer::div_floor(6, 3), 2);
    assert_eq!(integer::div_ceil(6, 3), 2);

    // Simultaneous floored integer division and modulus
    // 11 わる 3 = 3 あまり 2
    assert_eq!(integer::div_mod_floor(11, 3), (3, 2));
    // Simultaneous integer division and modulus
    assert_eq!(integer::div_rem(11, 3), (3, 2));

    // -11 わる 3
    assert_eq!(integer::div_mod_floor(-11, 3), (-4, 1));
    assert_eq!(integer::div_rem(-11, 3), (-3, -2));
    
    // 11 わる -3
    assert_eq!(integer::div_mod_floor(11, -3), (-4, -1));
    assert_eq!(integer::div_rem(11, -3), (-3, 2));

    // -11 わる -3
    assert_eq!(integer::div_mod_floor(-11, -3), (3, -2));
    assert_eq!(integer::div_rem(-11, -3), (3, -2));
    // -17 わる -5
    assert_eq!(integer::div_mod_floor(-17, -5), (3, -2));
    assert_eq!(integer::div_rem(-17, -5), (3, -2));

    // sqrt(x) x^(1/2)
    assert_eq!(integer::sqrt(2), 1);
    assert_eq!(Float::sqrt(2.0), 1.4142135623730951);
    assert_eq!(integer::sqrt(3), 1);
    assert_eq!(Float::sqrt(3.0), 1.7320508075688772);

    // cbrt(x) x^(1/3)
    assert_eq!(integer::cbrt(2), 1);
    assert_eq!(Float::cbrt(2.0), 1.259921049894873);

    // nth_root(x, n) x^(1/n)
    assert_eq!(integer::nth_root(16, 5), 1);

    // exp(x) e^x
    assert_eq!(Float::exp(2.0), 7.38905609893065);
    
    // exp2(x) 2^x
    assert_eq!(Float::exp2(2.0), 4.0);

    // ln(x) log_e(x)
    assert_eq!(Float::ln(2.0), 0.6931471805599453);
    
    // log(x, base) log_base(x)
    assert_eq!(Float::log(2.0, 3.0), 0.6309297535714574);
    

    // 複素数
    let z = Complex { re: 2.0, im: -1.0 };
    assert_eq!(
        z * z,
        Complex { re: 3.0, im: -4.0 }
    );

    // gcd, lcm
    assert_eq!(integer::gcd(12, 18), 6);
    assert_eq!(integer::lcm(12, 18), 36);
    
    // Binomial Coefficient n!/(k!*(n-k)!)
    assert_eq!(integer::binomial(10, 3), 120);

    random();

    // ndarray_sample();
}



fn random() {
    let mut v:Vec<i32> = vec![0; 10];
    for x in v.iter_mut() {
        *x = rand::random();
    }
    println!("random {:?}", v);
}


// use ndarray::prelude::*;
// use ndarray::{ShapeError, ErrorKind};

// fn ndarray_sample(){
//     let mut a = Array::zeros((0, 4));
//     a.append_row(aview1(&[0., 1., 2., 3.])).unwrap();
//     a.append_row(aview1(&[4., 5., 6., 7.])).unwrap();
//     assert_eq!(a.shape(), &[2, 4]);
// }