mod dasher;
use std::vec;
fn main() {
    let newvec = somei32();
        newvec..map(|vec| vec.into_iter().map(|value| value).collect());
}

fn somei32() -> Option<Vec<i32>> {
    let x = vec![1, 2, 3, 4, 5];
    let z: [u32; 10] = [0; 10];
    z.iter()
    .map(|value| value + 17)
    .collect();
    
    let y:Vec<i32> = x.into_iter()
     .map(|value| value)
     .collect();

    Some(x)
}
