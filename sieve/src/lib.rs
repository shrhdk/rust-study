pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }
    let mut list: Vec<u64> = (2..=upper_bound).collect();
    let mut primes: Vec<u64> = Vec::new();
    loop {
        let head = list.remove(0);
        primes.push(head);
        list.retain(|&i| i % head != 0);
        if head * head > upper_bound {
            primes.append(&mut list);
            return primes;
        }
    }
}
