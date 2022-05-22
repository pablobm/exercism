#[derive(PartialEq)]
#[derive(Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist<T>(l1: &[T], l2: &[T]) -> bool
    where T: PartialEq
{
    l1.is_empty() || l2.windows(l1.len()).any(|w| *w == *l1)
}

pub fn sublist<T>(l1: &[T], l2: &[T]) -> Comparison
    where T: PartialEq
{
    if *l1 == *l2 {
        Comparison::Equal
    }
    else if is_sublist(l1, l2) {
        Comparison::Sublist
    }
    else if is_sublist(l2, l1) {
        Comparison::Superlist
    }
    else {
        Comparison::Unequal
    }
}