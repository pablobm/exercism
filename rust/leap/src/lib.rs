pub fn is_leap_year(year: u64) -> bool {
    year.divisible_by(4) && (!year.divisible_by(100) || year.divisible_by(400))
}

trait DivisibleBy {
    fn divisible_by(&self, divisor: u64) -> bool;
}

impl DivisibleBy for u64 {
    fn divisible_by(&self, divisor: u64) -> bool {
        self % divisor == 0
    }
}
