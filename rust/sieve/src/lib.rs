pub fn primes_up_to(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();

    for potential_prime in 2..limit+1 {
        let mut factor_found = false;
        for potential_factor in 2..potential_prime {
            if potential_prime % potential_factor == 0 {
                factor_found = true;
                break;
            }
        }
        if ! factor_found {
            primes.push(potential_prime);
        }
    }

    primes
}
