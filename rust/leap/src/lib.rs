pub fn is_leap_year(year: i32) -> bool {
    year.divisible_by(4) && (!year.divisible_by(100) || year.divisible_by(400))
}

trait DivisibleBy {
    fn divisible_by(&self, divisor: i32) -> bool;
}

impl DivisibleBy for i32 {
    fn divisible_by(&self, divisor: i32) -> bool {
        self % divisor == 0
    }
}
