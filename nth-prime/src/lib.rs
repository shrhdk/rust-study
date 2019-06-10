fn is_prime(num: u64) -> bool {
    if num == 0 || num == 1 {
        false
    } else if num == 2 {
        true
    } else if num % 2 == 0 {
        false
    } else {
        for x in (3..num).step_by(2) {
            if num % x == 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(0), false);
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(10), false);
    assert_eq!(is_prime(11), true);
    assert_eq!(is_prime(12), false);
    assert_eq!(is_prime(13), true);
}

pub fn nth_prime(n: u64) -> u64 {
    assert!(n != 0, "n must be 1 or more, but got 0.");

    let mut i = 0u64;
    for x in 2u64.. {
        if is_prime(x) {
            i += 1;
            if i == n {
                return x;
            }
        }
    }

    panic!("this line is unreachable.")
}

#[test]
fn test_nth_prime() {
    assert_eq!(nth_prime(1), 2);
    assert_eq!(nth_prime(2), 3);
    assert_eq!(nth_prime(3), 5);
    assert_eq!(nth_prime(4), 7);
    assert_eq!(nth_prime(5), 11);
    assert_eq!(nth_prime(6), 13);
}

#[test]
#[should_panic]
fn test_0th_prime() {
    nth_prime(0);
}
