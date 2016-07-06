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
    if l1.len() == l2.len() {
        if *l1 == *l2 {
            Comparison::Equal
        }
        else {
            Comparison::Unequal
        }
    }
    else if l1.len() == 0 {
        Comparison::Sublist
    }
    else if l2.len() == 0 {
        Comparison::Superlist
    }
    else if l1.len() < l2.len() && l2.windows(l1.len()).any(|w| *w == *l1) {
        Comparison::Sublist
    }
    else if l1.len() > l2.len() && l1.windows(l2.len()).any(|w| *w == *l2) {
        Comparison::Superlist
    }
    else {
        Comparison::Unequal
    }
}
