#[derive(PartialEq)]
#[derive(Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T>(l1: &[T], l2: &[T]) -> Comparison
    where T: PartialEq
{
    if *l1 == *l2 {
        Comparison::Equal
    }
    else if l1.is_empty() || l2.windows(l1.len()).any(|w| *w == *l1) {
        Comparison::Sublist
    }
    else if l2.is_empty() || l1.windows(l2.len()).any(|w| *w == *l2) {
        Comparison::Superlist
    }
    else {
        Comparison::Unequal
    }
}
