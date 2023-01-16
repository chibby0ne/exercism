use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Hash + PartialEq + Eq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // Base case: They are equal
    if _first_list.eq(_second_list) {
        return Comparison::Equal;
    }

    // let mut first_map: HashMap<&T, u32> = HashMap::new();
    // for elem in _first_list {
    //     first_map.entry(elem).and_modify(|x| *x += 1).or_insert(1);
    // }

    // let mut second_map: HashMap<&T, u32> = HashMap::new();
    // for elem in _second_list {
    //     second_map.entry(elem).and_modify(|x| *x += 1).or_insert(1);
    // }

    // If there are no equal elements, then it's Unequal
    if !_second_list.iter().any(|x| _first_list.contains(x)) {
        return Comparison::Unequal;
    }

    // There's at least one common element between the list so they could be either super or
    // sublist
    if _first_list.len() < _second_list.len() {
        Comparison::Sublist
    } else {
        Comparison::Superlist
    }
}
