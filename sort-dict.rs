#[macro_use]
extern crate itertools;

use itertools::Itertools;
fn sortDict(d : &Vec<(i32, i32)>) -> Vec<&(i32, i32)>{
    d.iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
}

fn main(){
    assert_eq!(sortDict(&vec![(3,1), (2,2), (1,3)]), vec![&(1,3), &(2,2), &(3,1)]);
    assert_eq!(sortDict(&vec![(1,2),(2,4),(3,6)]), vec![&(3,6),&(2,4),&(1,2)]);
}
