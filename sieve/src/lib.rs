pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut list: Vec<u64> = (2..=upper_bound).collect();
    let mut primes: Vec<u64> = Vec::new();
    while let Some(head) = list.first().cloned() {
        primes.push(head);
        list.retain(|&i| i % head != 0);
        if head * head > upper_bound {
            primes.append(&mut list);
            break;
        }
    }
    primes
}
