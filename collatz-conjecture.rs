fn cj(n : i32, c : u32)->u32{
    if n == 1{
        c + 1
    }
    else if n & 1 == 0{
        cj(n / 2, c + 1)
    }
    else{
        cj(n * 3 + 1, c + 1)
    }
}

fn main(){
    assert_eq!(cj(20, 0), 8);
}
