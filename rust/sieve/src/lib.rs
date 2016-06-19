pub fn primes_up_to(limit: u32) -> Vec<u32> {
    (2..limit+1)
        .filter(is_prime)
        .collect()
}

fn is_prime(&num: &u32) -> bool {
    if num == 2 {
        return true;
    }

    (2..num.pseudo_sqrt()+1)
        .all(|x| num % x != 0)
}

trait Sqrt {
    fn pseudo_sqrt(&self) -> u32;
}

impl Sqrt for u32 {
    fn pseudo_sqrt(&self) -> u32 {
        (*self as f32).sqrt().round() as u32
    }
}
