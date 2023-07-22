mod dasher;
use std::{vec, collections::BTreeMap};
fn main() {
    let newvec = somei32();
        newvec.map(|vec| vec.into_iter().map(|value| value).collect::<Vec<_>>());
}

fn somei32() -> Option<Vec<i32>> {
    let x = vec![1, 2, 3, 4, 5];
    let mut m: BTreeMap<String, String> = BTreeMap::new();

    m.insert("die".to_string(), "shit".to_string());
    let yx = m.get("die");

    let y:Vec<i32> = x.into_iter().rev()
     .map(|value| value)
     .collect::<Vec<_>>();

    Some(x)
}
