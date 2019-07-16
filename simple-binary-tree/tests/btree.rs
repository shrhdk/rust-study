use rand::seq::SliceRandom;

use simple_binary_tree::BTreeSet;

#[test]
fn test_insert_increments_length() {
    let mut set = BTreeSet::<i32>::new();
    set.insert(1);
    assert_eq!(set.len(), 1, "set's length must be 1");
    set.insert(2);
    assert_eq!(set.len(), 2, "set's length must be 2");
}

#[test]
fn test_insert_duplicated_value_keeps_length() {
    let mut set = BTreeSet::<i32>::new();
    set.insert(1);
    assert_eq!(set.len(), 1, "set's length must be 1");
    set.insert(1);
    assert_eq!(set.len(), 1, "set's length must be 1");
}

#[test]
fn test_insert_unique_value_returns_true() {
    let mut set = BTreeSet::<i32>::new();
    assert_eq!(set.insert(1), true);
    assert_eq!(set.insert(2), true);
}

#[test]
fn test_insert_duplicated_value_returns_false() {
    let mut set = BTreeSet::<i32>::new();
    set.insert(1);
    assert_eq!(set.insert(1), false);
}

#[test]
fn test_is_empty() {
    let mut set = BTreeSet::<i32>::new();
    assert_eq!(set.is_empty(), true);
    set.insert(1);
    assert_eq!(set.is_empty(), false);
}

#[test]
fn test_clear() {
    let mut set = BTreeSet::<i32>::new();

    set.insert(1);
    set.insert(2);
    set.clear();

    assert_eq!(set.len(), 0);
    assert_eq!(set.contains(&1), false);
    assert_eq!(set.get(&1), None);
    assert_eq!(set.contains(&2), false);
    assert_eq!(set.get(&2), None);
}

#[test]
fn test_contains() {
    let mut set = BTreeSet::<i32>::new();

    // Insert to root
    // 10
    // /\
    assert_eq!(
        set.contains(&10),
        false,
        "set's does not contains 10 before insertion."
    );
    set.insert(10);
    assert_eq!(
        set.contains(&10),
        true,
        "set's contains 10 after insertion."
    );

    // Insert to left
    //  10
    //  /\
    // 5
    assert_eq!(
        set.contains(&5),
        false,
        "set's does not contains 5 before insertion."
    );
    set.insert(5);
    assert_eq!(set.contains(&5), true, "set's contains 5 after insertion.");

    // Insert to right
    //  10
    //  /\
    // 5 15
    assert_eq!(
        set.contains(&15),
        false,
        "set's does not contains 15 before insertion."
    );
    set.insert(15);
    assert_eq!(
        set.contains(&15),
        true,
        "set's contains 15 after insertion."
    );

    // Insert to left left
    //    10
    //    /\
    //   5 15
    //  /
    // 3
    assert_eq!(
        set.contains(&3),
        false,
        "set's does not contains 3 before insertion."
    );
    set.insert(3);
    assert_eq!(set.contains(&3), true, "set's contains 3 after insertion.");

    // Insert to left right
    //    10
    //    /\
    //   5 15
    //  /\
    // 3 7
    assert_eq!(
        set.contains(&7),
        false,
        "set's does not contains 7 before insertion."
    );
    set.insert(7);
    assert_eq!(set.contains(&7), true, "set's contains 7 after insertion.");

    // Insert to right right
    //    10
    //    /\
    //   5 15
    //  /\  \
    // 3 7   20
    assert_eq!(
        set.contains(&20),
        false,
        "set's does not contains 20 before insertion."
    );
    set.insert(20);
    assert_eq!(
        set.contains(&20),
        true,
        "set's contains 20 after insertion."
    );
}

#[test]
fn test_get() {
    let mut set = BTreeSet::<i32>::new();

    // Insert to root
    // 10
    // /\
    assert_eq!(
        set.get(&10),
        None,
        "set's does not contains 10 before insertion."
    );
    set.insert(10);
    assert_eq!(
        set.get(&10),
        Some(&10),
        "set's contains 10 after insertion."
    );

    // Insert to left
    //  10
    //  /\
    // 5
    assert_eq!(
        set.get(&5),
        None,
        "set's does not contains 5 before insertion."
    );
    set.insert(5);
    assert_eq!(set.get(&5), Some(&5), "set's contains 5 after insertion.");

    // Insert to right
    //  10
    //  /\
    // 5 15
    assert_eq!(
        set.get(&15),
        None,
        "set's does not contains 15 before insertion."
    );
    set.insert(15);
    assert_eq!(
        set.get(&15),
        Some(&15),
        "set's contains 15 after insertion."
    );

    // Insert to left left
    //    10
    //    /\
    //   5 15
    //  /
    // 3
    assert_eq!(
        set.get(&3),
        None,
        "set's does not contains 3 before insertion."
    );
    set.insert(3);
    assert_eq!(set.get(&3), Some(&3), "set's contains 3 after insertion.");

    // Insert to left right
    //    10
    //    /\
    //   5 15
    //  /\
    // 3 7
    assert_eq!(
        set.get(&7),
        None,
        "set's does not contains 7 before insertion."
    );
    set.insert(7);
    assert_eq!(set.get(&7), Some(&7), "set's contains 7 after insertion.");

    // Insert to right right
    //    10
    //    /\
    //   5 15
    //  /\  \
    // 3 7   20
    assert_eq!(
        set.get(&20),
        None,
        "set's does not contains 20 before insertion."
    );
    set.insert(20);
    assert_eq!(
        set.get(&20),
        Some(&20),
        "set's contains 20 after insertion."
    );
}

#[test]
fn test_insert_sequential_numbers() {
    let mut set = BTreeSet::<u32>::new();
    for N in 0..=10 {
        for n in 0..=N {
            set.insert(n);
        }
        println!();
        println!("insert 0 to {}", N);
        set.pretty_print();
    }
}

#[test]
fn test_insert_random_numbers() {
    // gen random numbers
    let mut vec: Vec<u32> = (1..=50).collect();
    let mut rng = rand::thread_rng();
    vec.shuffle(&mut rng);

    let mut set = BTreeSet::<u32>::new();
    for v in vec {
        set.insert(v);
    }

    set.pretty_print();
}
