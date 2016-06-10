pub fn square_of_sum(num: u32) -> u32 {
    (1..num+1).fold(0, |acc, n| acc + n).pow(2)
}

pub fn sum_of_squares(num: u32) -> u32 {
    (1..num+1).fold(0, |acc, n| acc + n.pow(2))
}

pub fn difference(num: u32) -> u32 {
    square_of_sum(num) - sum_of_squares(num)
}
