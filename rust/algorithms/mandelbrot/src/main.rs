extern crate num;
use num::Complex;
use std::str::FromStr;

fn main() {
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

// Parse a pair of floating-point numbers separated by a comma as a complex number.
fn parse_complex(s: &str) ->  Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex{ re, im }),
        None => None
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.00625"), 
               Some(Complex {re: 1.25, im: -0.00625}));
    assert_eq!(parse_complex(", -0.00625"), None);

}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}