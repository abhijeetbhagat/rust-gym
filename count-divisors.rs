fn main(){
    let mut input= String::new();
    std::io::stdin().read_line(&mut input);
    let v : Vec<u32> = input.split_whitespace().map(|_str| _str.parse::<u32>().unwrap_or(0)).collect();
    let mut cnt = 0;
    for i in v[0]..v[1]+1{
        if v[2] <= i && i % v[2] == 0{
            cnt += 1;
        }
    }

    println!("{}", cnt); 
}
