pub fn hamming_distance(s1: &str, s2: &str) -> Result<u32, &'static str> {
    if s1.len() != s2.len() {
        Err("inputs of different length")
    }
    else {
        let distance = s1.chars().zip(s2.chars()).fold(0,
          |acc, pair|
          match pair {
              (n1, n2) if n1 != n2 => acc + 1,
              _ => acc,
          }
        );
        Ok(distance)
    }
}
