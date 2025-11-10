use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut t = BTreeMap::new();
    h.iter().for_each(|(k, v)| {
        v.iter().for_each(|c| {
            t.insert(c.to_lowercase().next().unwrap(), *k);
        });
    });
    t
}
