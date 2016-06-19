pub fn primes_up_to(limit: u32) -> Vec<u32> {
    (2..limit+1)
        .filter(is_prime)
        .collect()
}

fn is_prime(&num: &u32) -> bool {
    if num == 2 {
        return true;
    }

    let limit = (num as f32).sqrt().ceil() as u32;
    (2..limit+1)
        .all(|x| num % x != 0)
}
