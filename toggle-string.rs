fn main(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);

    let mut v : Vec<u8> = input.trim().bytes().collect();
    for c in v.iter_mut(){
        if *c <= 90{
            *c += 32;
        } else {
            *c -= 32;
        }
    }

    println!("{}", String::from_utf8(v).unwrap());
}
