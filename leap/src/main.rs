use std::str::FromStr;

fn main() {
    let year_str = std::env::args()
        .skip(1)
        .next()
        .expect("please specify a year.");
    let year = u64::from_str(&year_str).expect("error parsing argument");

    if is_leap_year(year) {
        println!("The {} is a leap year.", year);
    } else {
        println!("The {} is not a leap year.", year);
    }
}

fn is_leap_year(year: u64) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

#[test]
fn test_is_leap_year() {
    assert_eq!(is_leap_year(1997), false);
    assert_eq!(is_leap_year(1996), true);
    assert_eq!(is_leap_year(1900), false);
    assert_eq!(is_leap_year(2000), true);
}
