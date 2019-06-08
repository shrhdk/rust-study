use std::str::FromStr;

fn main() {
    let year_str = match std::env::args().nth(1) {
        Some(year_str) => year_str,
        None => {
            eprintln!("please specify a year.");
            std::process::exit(1)
        }
    };
    let year = match u64::from_str(&year_str) {
        Ok(year) => year,
        Err(err) => {
            eprintln!("failed to parse '{}' as a number : {}", year_str, err);
            std::process::exit(1)
        }
    };

    if is_leap_year(year) {
        println!("The {} is a leap year.", year);
    } else {
        println!("The {} is not a leap year.", year);
    }
}

fn is_leap_year(year: u64) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

#[test]
fn test_is_leap_year() {
    assert_eq!(is_leap_year(1997), false);
    assert_eq!(is_leap_year(1996), true);
    assert_eq!(is_leap_year(1900), false);
    assert_eq!(is_leap_year(2000), true);
}
