use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut output = BTreeMap::new();
    for (score, words) in input {
        for word in words {
            output.insert(word.to_lowercase(), *score);
        }
    }
    output
}
