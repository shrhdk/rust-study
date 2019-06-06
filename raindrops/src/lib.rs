pub fn raindrops(num: u64) -> String {
    if num % (3 * 5 * 7) == 0 {
        "PlingPlangPlong".to_string()
    } else if num % (3 * 5) == 0 {
        "PlingPlang".to_string()
    } else if num % (3 * 7) == 0 {
        "PlingPlong".to_string()
    } else if num % (5 * 7) == 0 {
        "PlangPlong".to_string()
    } else if num % 3 == 0 {
        "Pling".to_string()
    } else if num % 5 == 0 {
        "Plang".to_string()
    } else if num % 7 == 0 {
        "Plong".to_string()
    } else {
        num.to_string()
    }
}

#[test]
fn test_raindrops() {
    assert_eq!(raindrops(1), "1");
    assert_eq!(raindrops(2), "2");
    assert_eq!(raindrops(3), "Pling");
    assert_eq!(raindrops(4), "4");
    assert_eq!(raindrops(5), "Plang");
    assert_eq!(raindrops(6), "Pling");
    assert_eq!(raindrops(7), "Plong");
    assert_eq!(raindrops(8), "8");
    assert_eq!(raindrops(9), "Pling");
    assert_eq!(raindrops(10), "Plang");
    assert_eq!(raindrops(11), "11");
    assert_eq!(raindrops(12), "Pling");
    assert_eq!(raindrops(13), "13");
    assert_eq!(raindrops(14), "Plong");
    assert_eq!(raindrops(15), "PlingPlang");
    assert_eq!(raindrops(16), "16");
    assert_eq!(raindrops(17), "17");
    assert_eq!(raindrops(18), "Pling");
    assert_eq!(raindrops(19), "19");
    assert_eq!(raindrops(20), "Plang");
    assert_eq!(raindrops(21), "PlingPlong");
    assert_eq!(raindrops(5 * 7), "PlangPlong");
    assert_eq!(raindrops(3 * 5 * 7), "PlingPlangPlong");
}
