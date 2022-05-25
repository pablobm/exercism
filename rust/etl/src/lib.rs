use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output = BTreeMap::new();
    for (score, letters) in input {
        for letter in letters {
            output.insert(letter.to_lowercase().next().unwrap(), *score);
        }
    }
    output
}
