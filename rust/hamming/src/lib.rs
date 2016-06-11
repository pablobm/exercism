pub fn hamming_distance(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        Err("inputs of different length")
    }
    else {
        let distance = s1.chars().zip(s2.chars())
            .filter(|&(n1, n2)| n1 != n2)
            .count();
        Ok(distance)
    }
}
