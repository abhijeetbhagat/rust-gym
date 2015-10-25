fn cookingTime(n : u32) -> u32{
    let r = n % 8;
    let q = n/8;
    q * 5 + if r == 0 {0} else {5}
}

fn main(){
    assert_eq!(cookingTime(0), 0);
    assert_eq!(cookingTime(5), 5);
    assert_eq!(cookingTime(8), 5);
    assert_eq!(cookingTime(10), 10);
}

