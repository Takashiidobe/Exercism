use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Eq + Hash + Clone>(
    first_list: &[T],
    second_list: &[T],
) -> Comparison {
    let first_set: HashSet<T> = first_list.iter().cloned().collect();
    let second_set: HashSet<T> = second_list.iter().cloned().collect();

    if first_set == second_set {
        Comparison::Equal
    } else if first_set.is_superset(&second_set) {
        Comparison::Superlist
    } else if first_set.is_subset(&second_set) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
