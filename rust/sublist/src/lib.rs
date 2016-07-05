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
    else if l1.len() < l2.len() {
        for wlen in l1.len()..l2.len() {
            for window in l2.windows(wlen) {
                if *window == *l1 {
                    return Comparison::Sublist
                }
            }
        }
        return Comparison::Unequal
    }
    else if l1.len() > l2.len() {
        for wlen in l2.len()..l1.len() {
            for window in l1.windows(wlen) {
                if *window == *l2 {
                    return Comparison::Superlist
                }
            }
        }
        return Comparison::Unequal
    }
    else {
        panic!("No idea of what's going on");
    }
}
