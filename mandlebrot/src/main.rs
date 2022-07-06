use num::Complex;

/// Determine if the number is in the mandlebrot set
/// If z leaves the circle of radius 2, then we return i,
/// the number of iterations it took to prove it was not 
/// in the set. Return None if we have no proof that the number
/// leaves the circle within limit itterations.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

use std::str::FromStr; 

/// Parse the string 's' as a coordinate pair, like '"400x600"' or '"1.0,0.5"'.
///
/// Specifically, 's' should have the form <left><sep><right>, where <sep> is 
/// the character given by the 'separator' argument, and <left> and <right> are
/// both strings that can be parsed by 'T::from_str'. 'separator' must be an
/// ASCII character 

/// If 's' has the proper form, return 'Some<(x, y)>'. If it doesn't parse
/// correctly, return 'None'.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test] 
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",           ','), None);
    assert_eq!(parse_pair::<i32>("10,",        ','), None);
    assert_eq!(parse_pair::<i32>(",10",        ','), None);
    assert_eq!(parse_pair::<i32>("10,20",      ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy",    ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",       'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",    'x'), Some((0.5, 1.5)));
}


/// Parse a pair of floating-point numbers seperated by a comma as a complex 
/// number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
               Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);

}
