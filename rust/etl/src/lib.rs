use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map: BTreeMap<char, i32> = BTreeMap::new();
    for (&p, vec) in h {
        for c in vec {
            map.insert(c.to_lowercase().next().unwrap(), p);
        }
    }
    map
}
