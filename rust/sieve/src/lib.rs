pub fn primes_up_to(limit: u32) -> Vec<u32> {
    (2..limit+1)
        .filter(is_prime)
        .collect()
}

fn is_prime(&num: &u32) -> bool {
    (2..num)
        .all(|x| num % x != 0)
}
